use structs::{request::Request, response::Response};
use tokio::net::UdpSocket;

pub mod language;
pub mod structs;

#[tokio::main]
async fn main() {
    let server: UdpSocket = UdpSocket::bind("0.0.0.0:53").await.unwrap();
    let mut buf: [u8; 1024] = [0; 1024];

    loop {
        let (len, addr) = server.recv_from(&mut buf).await.unwrap();

        let mut data: Vec<u8> = buf.to_vec();
        data.resize(len, 0);

        let mut request: Request = Request::new();
        request.with_bytes(data);

        let mut response: Response = Response::from(request);
        let data_response: Vec<u8> = response.create_byte_response();

        println!("{:?}", data_response);

        server.send_to(&data_response, addr).await.unwrap();
    }
}
