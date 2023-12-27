use std::error::Error;

use crate::domain::{body::DNSBody, header::DNSHeader, question::DNSQuestion};

use super::compression::decompress_domain_name;

pub fn parse_body(input: &[u8]) -> Result<DNSBody, Box<dyn Error>> {
    let (header, remainder) = parse_header(input)?;
    let (questions, _) = parse_questions(remainder, header.qdcount, input)?;
    Ok(DNSBody {
        header,
        questions,
        ..Default::default()
    })
}

fn parse_header(input: &[u8]) -> Result<(DNSHeader, &[u8]), Box<dyn Error>> {
    let header_slice: &[u8] = &input[..12];
    let header_array: Result<[u8; 12], _> = header_slice.try_into();

    match header_array {
        Ok(header_array) => Ok((DNSHeader::from_bytes(&header_array), &input[12..])),
        Err(_) => Err("Failed to convert header slice into array".into()),
    }
}

fn parse_questions<'a>(
    input: &'a [u8],
    count: u16,
    entire_message: &[u8],
) -> Result<(Vec<DNSQuestion>, &'a [u8]), Box<dyn Error>> {
    let mut questions = Vec::new();
    let mut remainder = input;

    for _ in 0..count {
        let (question, r) = parse_question(input, entire_message);
        questions.push(question);
        remainder = r;
    }
    Ok((questions, remainder))
}

/// This needs to take in a reference to the entire message bytearray in order to decompress domain
/// names
fn parse_question<'a>(input: &'a [u8], entire_message: &[u8]) -> (DNSQuestion, &'a [u8]) {
    // determine question boundaries
    let end_of_domain_name_idx = input
        .iter()
        .position(|b| *b == 0_u8)
        .expect("input is not a valid question");
    let end_of_question_idx = end_of_domain_name_idx + 4;
    // decompress domain name
    let decompressed_domain_name =
        decompress_domain_name(&input[..end_of_domain_name_idx + 1], entire_message);
    // build question and return
    (
        DNSQuestion::from_bytes(&input[..end_of_question_idx + 1]),
        &input[end_of_question_idx + 1..],
    )
}
