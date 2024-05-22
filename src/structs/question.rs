#[derive(Debug)]
enum Qtype {
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
}

#[derive(Debug)]
enum Qclass {
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
}

#[derive(Debug)]
pub struct Question {
    qname: String,
    qtype: Qtype,
    qclass: Qclass,
}

impl Question {
    pub fn new() -> Self {
        Self {
            qname: String::new(),
            qtype: Qtype::Unknow,
            qclass: Qclass::Unknow,
        }
    }

    pub fn with_bytes(mut bytes: Vec<u8>) -> Self {
        let bytes_len = bytes.len();
        let mut bytes_clone = bytes.clone();

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
    let mut result = String::new();

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
