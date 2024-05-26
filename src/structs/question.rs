use std::str::Split;

#[derive(Debug, PartialEq, Clone)]
pub enum Qtype {
    A = 1,
    NS = 2,
    MD = 3,
    MF = 4,
    CNAME = 5,
    SOA = 6,
    MB = 7,
    MG = 8,
    MR = 9,
    NULL = 10,
    WKS = 11,
    PTR = 12,
    HINFO = 13,
    MINFO = 14,
    MX = 15,
    TXT = 16,
    AAAA = 28,
    Unknow,
}

impl Qtype {
    fn with_bytes(bytes: u16) -> Self {
        match bytes {
            1 => Self::A,
            2 => Self::NS,
            3 => Self::MD,
            4 => Self::MF,
            5 => Self::CNAME,
            6 => Self::SOA,
            7 => Self::MB,
            8 => Self::MG,
            9 => Self::MR,
            10 => Self::NULL,
            11 => Self::WKS,
            12 => Self::PTR,
            13 => Self::HINFO,
            14 => Self::MINFO,
            15 => Self::MX,
            16 => Self::TXT,
            28 => Self::AAAA,
            _ => Self::Unknow,
        }
    }

    pub fn with_string(string: String) -> Self {
        match string.as_str() {
            "A" => Self::A,
            "NS" => Self::NS,
            "MD" => Self::MD,
            "MF" => Self::MF,
            "CNAME" => Self::CNAME,
            "SOA" => Self::SOA,
            "MB" => Self::MB,
            "MG" => Self::MG,
            "MR" => Self::MR,
            "NULL" => Self::NULL,
            "WKS" => Self::WKS,
            "PTR" => Self::PTR,
            "HINFO" => Self::HINFO,
            "MINFO" => Self::MINFO,
            "MX" => Self::MX,
            "TXT" => Self::TXT,
            "AAAA" => Self::AAAA,
            _ => Self::Unknow,
        }
    }

    pub fn to_bytes(&self) -> u16 {
        match self {
            Self::A => 0x1,
            Self::NS => 0x2,
            Self::MD => 0x3,
            Self::MF => 0x4,
            Self::CNAME => 0x5,
            Self::SOA => 0x6,
            Self::MB => 0x7,
            Self::MG => 0x8,
            Self::MR => 0x9,
            Self::NULL => 0xA,
            Self::WKS => 0xB,
            Self::PTR => 0xC,
            Self::HINFO => 0xD,
            Self::MINFO => 0xE,
            Self::MX => 0xF,
            Self::TXT => 0x10,
            Self::AAAA => 0x1C,
            // TODO
            Self::Unknow => 0x99,
        }
    }
}

#[derive(Debug)]
pub enum Qclass {
    IN = 1,
    Unknow,
}

impl Qclass {
    fn with_bytes(bytes: u16) -> Self {
        match bytes {
            1 => Self::IN,
            _ => Self::Unknow,
        }
    }

    pub fn to_bytes(&self) -> u16 {
        match self {
            Self::IN => 0x1,
            // TODO
            Self::Unknow => 0x99,
        }
    }
}

#[derive(Debug)]
pub struct Question {
    pub qname: String,
    pub qtype: Qtype,
    pub qclass: Qclass,
}

impl Question {
    pub fn new() -> Self {
        Self {
            qname: String::new(),
            qtype: Qtype::Unknow,
            qclass: Qclass::Unknow,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let domain_splitted: Split<&str> = self.qname.split(".");
        let mut domain_bytes: Vec<u8> = Vec::new();

        for domain_split in domain_splitted {
            domain_bytes.push(domain_split.len().try_into().unwrap());

            for c in domain_split.chars() {
                domain_bytes.push(c as u8);
            }
        }
        domain_bytes.push(0x0);

        domain_bytes.extend_from_slice(&self.qtype.to_bytes().to_be_bytes());
        domain_bytes.extend_from_slice(&self.qclass.to_bytes().to_be_bytes());

        return domain_bytes;
    }

    pub fn with_bytes(bytes: Vec<u8>) -> Self {
        let bytes_len: usize = bytes.len();
        let mut bytes_clone: Vec<u8> = bytes.clone();

        Self {
            qname: decode_string(&mut bytes_clone),
            qtype: Qtype::with_bytes(u16::from_be_bytes([
                bytes[bytes_len - 4],
                bytes[bytes_len - 3],
            ])),
            qclass: Qclass::with_bytes(u16::from_be_bytes([
                bytes[bytes_len - 2],
                bytes[bytes_len - 1],
            ])),
        }
    }
}

fn decode_string(bytes: &mut Vec<u8>) -> String {
    let mut result: String = String::new();

    loop {
        let last_index: usize = (bytes[0] + 1).try_into().unwrap();
        let name: String = String::from_utf8_lossy(&bytes[1..last_index]).to_string();

        result += &name;

        bytes.drain(0..last_index);

        if bytes[0] == 0x00 {
            break;
        }

        result += ".";
    }

    return result;
}
