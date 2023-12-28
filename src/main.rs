use nom::AsBytes;
use std::env;

use crate::{
    domain::{answer::DNSAnswer, body::DNSBody, header::DNSHeader, question::DNSQuestion},
    parsing::{arguments::parse_arguments_for_resolver, body::parse_body},
};
use std::net::{SocketAddr, UdpSocket};

mod domain;
mod parsing;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let maybe_resolver_address = parse_arguments_for_resolver(&arguments);

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                let rcv = &buf[0..size];
                handle_request(rcv, source, &udp_socket, maybe_resolver_address);
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}

fn handle_request(
    data: &[u8],
    source_address: SocketAddr,
    socket: &UdpSocket,
    resolver_address: Option<SocketAddr>,
) {
    match parse_body(data) {
        Ok(request_body) => {
            let answers: Vec<DNSAnswer>;
            if let Some(address) = resolver_address {
                answers = resolve_request(&request_body, address);
            } else {
                answers = request_body.questions.iter().map(build_answer).collect();
            }
            let questions = request_body.questions;
            let header = DNSHeader {
                qr: 1,
                rcode: match request_body.header.opcode {
                    0 => 0,
                    _ => 4,
                },
                ancount: answers.len() as u16,
                ..request_body.header
            };
            let response = DNSBody {
                header,
                questions,
                answers,
            };
            socket
                .send_to(response.to_bytes().as_bytes(), source_address)
                .expect("Failed to send response");
        }
        Err(e) => {
            eprintln!("Error parsing request body: {}", e);
        }
    }
}

fn resolve_request(request_body: &DNSBody, resolver_address: SocketAddr) -> Vec<DNSAnswer> {
    // divide request into single question bodies
    let sq_bodies: Vec<DNSBody> = request_body
        .questions
        .iter()
        .map(|question| DNSBody {
            header: DNSHeader {
                qdcount: 1,
                ..request_body.header
            },
            questions: vec![question.clone()],
            answers: Vec::new(),
        })
        .collect();
    // for each one, send to resolver, parse and append response
    let mut answers = Vec::new();
    let socket = UdpSocket::bind("127.0.0.1:2054").expect("Failed to bind to address");
    for sq_body in sq_bodies {
        if let Some(response) = query_resolver(sq_body, &socket, resolver_address) {
            answers.extend(response.answers);
        }
    }
    // return answers
    answers
}

fn query_resolver(
    request_body: DNSBody,
    socket: &UdpSocket,
    resolver_address: SocketAddr,
) -> Option<DNSBody> {
    match socket.send_to(request_body.to_bytes().as_bytes(), resolver_address) {
        Ok(size) => {
            println!("Sent {} bytes to resolver", size);
            let mut buf = [0_u8; 512];
            loop {
                match socket.recv_from(&mut buf) {
                    Ok((size, source)) => {
                        println!("Received {} bytes from resolver at {}", size, source);
                        let rcv = &buf[0..size];
                        match parse_body(rcv) {
                            Ok(response_body) => {
                                return Some(response_body);
                            }
                            Err(e) => {
                                eprintln!("Error parsing request body: {}", e);
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error receiving data: {}", e);
                        break;
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error sending query to resolver: {}", e);
        }
    }
    return None;
}

fn build_answer(question: &DNSQuestion) -> DNSAnswer {
    DNSAnswer {
        name: question.domain_name.to_owned(),
        record_type: 1,
        class: 1,
        ttl: 60,
        length: 4,
        rdata: vec![8, 8, 8, 8],
    }
}
