use crate::parsing::label_encoding::{decode_label, encode_label};

#[derive(Debug)]
pub struct DNSQuestion {
    /// A domain name, represented as a sequence of "labels". Labels are encoded as
    /// <length><content>, where <length> is a single byte that specifies the length of the label,
    /// and <content> is the actual content of the label. The sequence of labels is terminated by a
    /// null byte (\x00). google.com is encoded as \x06google\x03com\x00
    pub domain_name: String,

    /// 2-byte int; the type of record (1 for an A record, 5 for a CNAME record etc.)
    pub query_type: u16,

    /// 2-byte int; usually set to 1
    pub query_class: u16,
}

impl DNSQuestion {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut byte_array = Vec::new();

        // convert domain name into byte_array
        byte_array.append(&mut encode_label(&self.domain_name));

        // append query type and query class to byte array
        byte_array.extend_from_slice(&self.query_type.to_be_bytes());
        byte_array.extend_from_slice(&self.query_class.to_be_bytes());

        byte_array
    }

    pub fn from_bytes(input_bytes: &[u8]) -> Self {
        let mut parts = input_bytes.splitn(2, |b| *b == 0_u8);
        let encoded_label = parts.next().unwrap();
        let remainder = parts.next().unwrap();
        let domain_name = decode_label(encoded_label);
        let query_type = (remainder[0] as u16) << 8 | remainder[1] as u16;
        let query_class = (remainder[2] as u16) << 8 | remainder[3] as u16;
        DNSQuestion {
            domain_name,
            query_type,
            query_class,
        }
    }
}
