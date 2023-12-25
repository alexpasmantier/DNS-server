use crate::{DNSHeader, DNSQuestion};

use super::answer::DNSAnswer;

#[derive(Debug, Default)]
pub struct DNSBody {
    pub header: DNSHeader,
    pub questions: Vec<DNSQuestion>,
    pub answers: Vec<DNSAnswer>, // more to come...
}
impl DNSBody {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut byte_array = Vec::new();
        // add header as vec
        byte_array.append(&mut self.header.to_bytes().to_vec());
        // add questions
        if self.questions.len() > 0 {
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
        }
        // add answers
        if self.answers.len() > 0 {
            byte_array.append(
                &mut self
                    .answers
                    .iter()
                    .map(|q| q.to_bytes())
                    .reduce(|acc, e| {
                        let mut c = acc;
                        c.extend(e.iter());
                        c
                    })
                    .expect("Unable to convert answers to bytearray"),
            );
        }
        byte_array
    }
}
