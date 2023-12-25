use crate::parsing::label_encoding::encode_label;

#[derive(Default, Debug)]
pub struct DNSAnswer {
    pub name: String,
    pub record_type: u16,
    pub class: u16,
    pub ttl: u32,
    pub length: u16,
    pub data: Vec<u8>,
}

impl DNSAnswer {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut byte_array = Vec::new();

        byte_array.append(&mut encode_label(&self.name));
        byte_array.extend_from_slice(&self.record_type.to_be_bytes());
        byte_array.extend_from_slice(&self.class.to_be_bytes());
        byte_array.extend_from_slice(&self.ttl.to_be_bytes());
        byte_array.extend_from_slice(&self.length.to_be_bytes());
        byte_array.extend(&self.data);

        byte_array
    }
}
