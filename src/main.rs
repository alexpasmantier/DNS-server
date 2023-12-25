use nom::AsBytes;

use crate::protocol::{body::DNSBody, header::DNSHeader, question::DNSQuestion};
use std::net::UdpSocket;

mod protocol;
mod utils;

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                let _received_data = String::from_utf8_lossy(&buf[0..size]);
                println!("Received {} bytes from {}", size, source);

                // TODO: replace these with a proper constructor
                let response_body = DNSBody {
                    header: DNSHeader {
                        id: 1234,
                        qr: 1,
                        qdcount: 1,
                        ..Default::default()
                    },
                    questions: vec![DNSQuestion {
                        domain_name: "codecrafters.io".to_string(),
                        query_type: 1,
                        query_class: 1,
                    }],
                };
                let response = response_body.to_bytes();
                let r = response.as_bytes();
                udp_socket
                    .send_to(r, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
