use std::net::UdpSocket;

use nom::AsBytes;

#[derive(Default)]
pub struct DNSHeader {
    /// 16 bits
    /// A random ID assigned to query packets. Response packets must reply with the same ID. Expected: 1234.
    id: u16,

    /// 1 bit
    /// 1 for a reply packet, 0 for a question packet. Expected: 1.
    qr: u8,

    /// 4 bits
    /// Specifies the kind of query in a message. Expected: 0.
    opcode: u8,

    /// 1 bit
    /// 1 if the responding server "owns" the domain queried, i.e., it's authoritative. Expected: 0.
    aa: u8,

    /// 1 bit
    /// 1 if the message is larger than 512 bytes. Always 0 in UDP responses. Expected: 0.
    tc: u8,

    /// 1 bit
    /// Sender sets this to 1 if the server should recursively resolve this query, 0 otherwise. Expected: 0.
    rd: u8,

    /// 1 bit
    /// Server sets this to 1 to indicate that recursion is available. Expected: 0.
    ra: u8,

    /// 3 bits
    /// Used by DNSSEC queries. At inception, it was reserved for future use. Expected: 0.
    z: u8,

    /// 4 bits
    /// Response code indicating the status of the response. Expected: 0.
    rcode: u8,

    /// 16 bits
    /// Number of questions in the Question section. Expected: 0.
    qdcount: u16,

    /// 16 bits
    /// Number of records in the Answer section. Expected: 0.
    ancount: u16,

    /// 16 bits
    /// Number of records in the Authority section. Expected: 0.
    nscount: u16,

    /// 16 bits
    /// Number of records in the Additional section. Expected: 0.
    arcount: u16,
}

impl DNSHeader {
    pub fn to_bytearray(&self) -> [u8; 12] {
        let mut byte_array = [0; 12];
        // convert header as two bytes
        byte_array[..2].clone_from_slice(&self.id.to_be_bytes());

        // convert {qr, opcode, aa, tc, rd} into 1 byte
        let qr_opcode_aa_tc_rd =
            (self.qr << 7) | (self.opcode << 3) | (self.aa << 2) | (self.tc << 1) | self.rd;
        byte_array[2] = qr_opcode_aa_tc_rd;

        // convert {ra, z, rcode} into 1 byte
        let ra_z_rcode = (self.ra << 7) | (self.z << 4) | self.rcode;
        byte_array[3] = ra_z_rcode;

        // convert qdcount into 2 bytes
        byte_array[4..6].clone_from_slice(&self.qdcount.to_be_bytes());

        // convert ancount into 2 bytes
        byte_array[4..6].clone_from_slice(&self.ancount.to_be_bytes());

        // convert nscount into 2 bytes
        byte_array[4..6].clone_from_slice(&self.nscount.to_be_bytes());

        // convert arcount into 2 bytes
        byte_array[4..6].clone_from_slice(&self.arcount.to_be_bytes());

        // println!("byte_array is {:?}", byte_array);
        byte_array
    }
}

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    let header = DNSHeader {
        id: 1234,
        qr: 1,
        ..Default::default()
    };

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                let _received_data = String::from_utf8_lossy(&buf[0..size]);
                println!("Received {} bytes from {}", size, source);
                let response = header.to_bytearray();
                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
