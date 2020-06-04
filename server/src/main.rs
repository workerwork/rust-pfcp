use messages::Message;
use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8888").unwrap();
    let mut buf = [0u8; 65535];
    loop {
        let (amt, src) = socket.recv_from(&mut buf).unwrap();
        println!("received {} bytes from: {:?}", amt, src);
        println!("{:?}", &buf[..amt]);
        Message::parse(&mut buf[..amt]).pack();
    }
}
