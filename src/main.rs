use nom::AsBytes;

use crate::{
    domain::{answer::DNSAnswer, body::DNSBody, header::DNSHeader, question::DNSQuestion},
    parsing::body::parse_body,
};
use std::net::UdpSocket;

mod domain;
mod parsing;

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                let _received_data = String::from_utf8_lossy(&buf[0..size]);
                let rcv = &buf[0..size];
                // println!("Received {} bytes from {}", size, source);

                if let Ok(received_body) = parse_body(rcv) {
                    let received_header = received_body.header;
                    // println!("RECEIVED HEADER {:?}", received_header);

                    let answers: Vec<DNSAnswer> =
                        received_body.questions.iter().map(build_answer).collect();

                    let response_body = DNSBody {
                        header: DNSHeader {
                            id: received_header.id,
                            qr: 1,
                            opcode: received_header.opcode,
                            aa: 0,
                            tc: 0,
                            rd: received_header.rd,
                            ra: 0,
                            z: 0,
                            rcode: match received_header.opcode {
                                0 => 0,
                                _ => 4,
                            },
                            qdcount: received_header.qdcount,
                            ancount: 1,
                            nscount: 0,
                            arcount: 0,
                        },
                        questions: received_body.questions,
                        answers,
                    };
                    // println!("RESPONSE BODY {:?}", response_body);
                    let response = response_body.to_bytes();
                    let r = response.as_bytes();
                    // print!("RESPONSE: {:?}", r);
                    udp_socket
                        .send_to(r, source)
                        .expect("Failed to send response");
                } else {
                    eprintln!("Error parsing input")
                }
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}

fn build_answer(question: &DNSQuestion) -> DNSAnswer {
    DNSAnswer {
        name: question.domain_name.to_owned(),
        record_type: 1,
        class: 1,
        ttl: 60,
        length: 4,
        data: vec![8, 8, 8, 8],
    }
}
