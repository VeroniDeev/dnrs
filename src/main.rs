use structs::{
    header::Header,
    question::{Qclass, Qtype, Question},
    request::Request,
    response::Response,
};
use tokio::net::UdpSocket;

pub mod language;
pub mod structs;

#[tokio::main]
async fn main() {
    let server = UdpSocket::bind("0.0.0.0:53").await.unwrap();
    let mut buf = [0; 1024];

    loop {
        let (len, addr) = server.recv_from(&mut buf).await.unwrap();

        let mut data = buf.to_vec();
        data.resize(len, 0);

        let mut request = Request::new();
        request.with_bytes(data);

        let mut response = Response::from(request);
        let data_response = response.create_byte_response();

        println!("{:?}", data_response);

        server.send_to(&data_response, addr).await.unwrap();
    }
}
