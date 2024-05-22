#[derive(Debug)]
pub struct Header {
    id: u16,
    flags: u16,
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
}

impl Header {
    pub fn new() -> Self {
        Self {
            id: 0,
            flags: 0,
            qdcount: 0,
            ancount: 0,
            nscount: 0,
            arcount: 0,
        }
    }
    pub fn with_bytes(bytes: Vec<u8>) -> Self {
        Self {
            id: u16::from_be_bytes([bytes[0], bytes[1]]),
            flags: u16::from_be_bytes([bytes[2], bytes[3]]),
            qdcount: u16::from_be_bytes([bytes[4], bytes[5]]),
            ancount: u16::from_be_bytes([bytes[6], bytes[7]]),
            nscount: u16::from_be_bytes([bytes[8], bytes[9]]),
            arcount: u16::from_be_bytes([bytes[10], bytes[10]]),
        }
    }
}
