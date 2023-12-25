use std::error::Error;

use crate::domain::{body::DNSBody, header::DNSHeader, question::DNSQuestion};

pub fn parse_body(input: &[u8]) -> Result<DNSBody, Box<dyn Error>> {
    let (header, remainder) = parse_header(input)?;
    let (questions, _) = parse_questions(remainder, header.qdcount)?;
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

fn parse_questions(input: &[u8], count: u16) -> Result<(Vec<DNSQuestion>, &[u8]), Box<dyn Error>> {
    let mut questions = Vec::new();
    let mut remainder = input;

    for _ in 0..count {
        let (question, r) = parse_question(input);
        questions.push(question);
        remainder = r;
    }
    Ok((questions, remainder))
}

fn parse_question(input: &[u8]) -> (DNSQuestion, &[u8]) {
    let end_of_domain_name_idx = input
        .iter()
        .position(|b| *b == 0_u8)
        .expect("input is not a valid question");
    let end_of_question_idx = end_of_domain_name_idx + 4;
    (
        DNSQuestion::from_bytes(&input[..end_of_question_idx + 1]),
        &input[end_of_question_idx + 1..],
    )
}
