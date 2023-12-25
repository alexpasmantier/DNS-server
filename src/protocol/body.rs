use crate::{DNSHeader, DNSQuestion};

#[derive(Debug)]
pub struct DNSBody {
    pub header: DNSHeader,
    pub questions: Vec<DNSQuestion>,
    // more to come...
}
impl DNSBody {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut byte_array = Vec::new();
        // add header as vec
        byte_array.append(&mut self.header.to_bytes().to_vec());
        // add questions
        byte_array.append(
            &mut self
                .questions
                .iter()
                .map(|q| q.to_bytes())
                .reduce(|acc, e| {
                    let mut c = acc;
                    c.extend(e.iter());
                    c
                })
                .expect("Unable to convert questions to bytearray"),
        );
        byte_array
    }
}
