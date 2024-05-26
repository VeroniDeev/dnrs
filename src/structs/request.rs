use super::{header::Header, question::Question};

#[derive(Debug)]
pub struct Request {
    pub header: Header,
    pub question: Question,
}

impl Request {
    pub fn new() -> Self {
        Self {
            header: Header::new(),
            question: Question::new(),
        }
    }
    pub fn with_bytes(&mut self, bytes: Vec<u8>) {
        let header_bytes: Vec<u8> = bytes[0..12].to_vec();
        self.header = Header::with_bytes(header_bytes);

        let question_bytes = bytes[12..].to_vec();
        self.question = Question::with_bytes(question_bytes);
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend(self.header.to_bytes());
        bytes.extend(self.question.to_bytes());

        return bytes;
    }
}
