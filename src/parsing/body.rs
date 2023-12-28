use std::error::Error;

use crate::domain::{answer::DNSAnswer, body::DNSBody, header::DNSHeader, question::DNSQuestion};

use super::compression::decompress_domain_name;

pub fn parse_body(input: &[u8]) -> Result<DNSBody, Box<dyn Error>> {
    let (header, remainder) = parse_header(input)?;
    let (questions, remainder) = parse_questions(remainder, header.qdcount, input)?;
    let (answers, _) = parse_answers(remainder, header.ancount)?;
    Ok(DNSBody {
        header,
        questions,
        answers,
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
        let (question, r) = parse_question(remainder, entire_message);
        questions.push(question);
        remainder = r;
    }
    Ok((questions, remainder))
}

/// This needs to take in a reference to the entire message bytearray in order to decompress domain
/// names
fn parse_question<'a>(input: &'a [u8], entire_message: &[u8]) -> (DNSQuestion, &'a [u8]) {
    // decompress domain name
    let (decompressed_domain_name, remainder) = decompress_domain_name(&input, entire_message);
    // build question and return
    (
        DNSQuestion::from_bytes(&[&decompressed_domain_name, &remainder[..4]].concat()),
        &remainder[4..],
    )
}

fn parse_answers(input: &[u8], count: u16) -> Result<(Vec<DNSAnswer>, &[u8]), Box<dyn Error>> {
    let mut answers = Vec::new();
    let mut remainder = input;

    for _ in 0..count {
        let (answer, r) = parse_answer(remainder);
        answers.push(answer);
        remainder = r;
    }
    Ok((answers, remainder))
}

fn parse_answer(input: &[u8]) -> (DNSAnswer, &[u8]) {
    let mut parts = input.splitn(2, |b| *b == 0_u8);
    let encoded_domain_name = parts.next().unwrap();
    let remainder = parts.next().unwrap();

    (
        DNSAnswer::from_bytes(&[encoded_domain_name, &[0_u8], &remainder[..14]].concat()),
        &remainder[14..],
    )
}
