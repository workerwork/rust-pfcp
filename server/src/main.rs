use messages::*;
use std::net::UdpSocket;

//use std::io::Cursor;
//use byteorder::{BigEndian, ReadBytesExt};

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8888").unwrap();
    //let mut buf = vec![0u8; 65535];
    let mut buf = [0u8; 65535];
    //let mut buf = Vec::new();
    loop {
        let (amt, src) = socket.recv_from(&mut buf).unwrap();
        println!("received {} bytes from: {:?}", amt, src);
        println!("{:?}", &buf[..amt]);
        //println!("{:#?}", header::Header::parse(&buf[..amt]));
        let mut header = header::Header::new();
        println!("{:?}", parse_header(&buf[..amt], &mut header));
        println!("{:#?}", header);
        //let mut rdr = Cursor::new(&buf);
        //assert_eq!(517, rdr.read_u8::<BigEndian>().unwrap());
    }
}
