use crate::language::{analyse, file_finder};

use super::request::Request;

#[derive(Debug)]
pub struct Response {
    pub request: Request,
}

impl Response {
    pub fn from(request: Request) -> Self {
        Self { request }
    }

    fn domain_to_bytes(&self, domain: String) -> Vec<u8> {
        let domain_splitted = domain.split(".");
        let mut domain_bytes: Vec<u8> = Vec::new();

        for domain_split in domain_splitted {
            domain_bytes.push(domain_split.len().try_into().unwrap());

            for c in domain_split.chars() {
                domain_bytes.push(c as u8);
            }
        }
        domain_bytes.push(0x0);

        return domain_bytes;
    }

    fn ip_to_bytes(&self, ip_array: Vec<&str>) -> Vec<u8> {
        let mut ip_bytes: Vec<u8> = Vec::new();

        for ip in ip_array {
            ip_bytes.push(ip.parse::<u8>().unwrap());
        }

        return ip_bytes;
    }

    pub fn create_byte_response(&mut self) -> Vec<u8> {
        let domain = self.request.question.qname.clone();

        let source = file_finder(domain.clone());

        if source == None {
            self.request.header.flags = 0x8183;
            return self.request.to_bytes();
        }

        let analyse = analyse(source.unwrap()).unwrap();

        let ttl = analyse.get("ttl").unwrap().clone().as_int().unwrap();
        let dns_type = analyse.get("type").unwrap().clone().as_qtype().unwrap();
        let ip = analyse.get("ip").unwrap().clone().as_str().unwrap();

        self.request.header.flags = 0x8180;

        if dns_type != self.request.question.qtype {
            return self.request.to_bytes();
        }

        self.request.header.ancount = 0x0001;
        let mut array: Vec<u8> = Vec::new();

        let domain_array: Vec<u8> = self.domain_to_bytes(domain);
        array.extend(domain_array);

        array.extend_from_slice(&dns_type.to_bytes().to_be_bytes());
        array.extend_from_slice(&self.request.question.qclass.to_bytes().to_be_bytes());
        array.extend_from_slice(&ttl.to_be_bytes());

        let ip_array: Vec<&str> = ip.split(".").collect();
        let ip_size: u16 = ip_array.len().try_into().unwrap();

        array.extend_from_slice(&ip_size.to_be_bytes());
        array.extend(self.ip_to_bytes(ip_array));

        let mut bytes = self.request.to_bytes();

        bytes.extend(array);

        return bytes;
    }
}
