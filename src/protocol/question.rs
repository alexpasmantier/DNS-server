use crate::utils::label_encoding::to_encoded_label;

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
        byte_array.append(&mut to_encoded_label(&self.domain_name));

        // append query type and query class to byte array
        byte_array.extend_from_slice(&self.query_type.to_be_bytes());
        byte_array.extend_from_slice(&self.query_class.to_be_bytes());

        byte_array
    }
}
