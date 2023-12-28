use crate::parsing::label_encoding::{decode_label, encode_label};

#[derive(Default, Debug)]
pub struct DNSAnswer {
    pub name: String,
    pub record_type: u16,
    pub class: u16,
    pub ttl: u32,
    pub length: u16,
    pub rdata: Vec<u8>,
}

impl DNSAnswer {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut byte_array = Vec::new();

        byte_array.append(&mut encode_label(&self.name));
        byte_array.extend_from_slice(&self.record_type.to_be_bytes());
        byte_array.extend_from_slice(&self.class.to_be_bytes());
        byte_array.extend_from_slice(&self.ttl.to_be_bytes());
        byte_array.extend_from_slice(&self.length.to_be_bytes());
        byte_array.extend(&self.rdata);

        byte_array
    }

    pub fn from_bytes(input_bytes: &[u8]) -> Self {
        let mut parts = input_bytes.splitn(2, |b| *b == 0_u8);
        let encoded_label = parts.next().unwrap();
        let remainder = parts.next().unwrap();
        let name = decode_label(encoded_label);
        let record_type = (remainder[0] as u16) << 8 | remainder[1] as u16;
        let class = (remainder[2] as u16) << 8 | remainder[3] as u16;
        let ttl = (remainder[4] as u32) << 24
            | (remainder[5] as u32) << 16
            | (remainder[6] as u32) << 8
            | remainder[7] as u32;
        let length = (remainder[8] as u16) << 8 | remainder[9] as u16;
        let rdata: Vec<u8> = remainder[10..14].to_vec();
        DNSAnswer {
            name,
            record_type,
            class,
            ttl,
            length,
            rdata,
        }
    }
}
