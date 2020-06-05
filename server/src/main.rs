use messages::Message;
use std::net::UdpSocket;

extern crate yaml_rust;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use yaml_rust::yaml;

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut f = File::open(&args[1]).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let docs = yaml::YamlLoader::load_from_str(&s).unwrap();

    let ip = &docs[0]["servers"][0]["server"]["ip"].as_str().unwrap();
    let port = &docs[0]["servers"][0]["server"]["port"].as_str().unwrap();

    let addr = format!("{}:{}", ip, port);

    let socket = UdpSocket::bind(addr).unwrap();
    let mut buf = [0u8; 65535];
    loop {
        let (amt, src) = socket.recv_from(&mut buf).unwrap();
        println!("received {} bytes from: {:?}", amt, src);
        println!("{:?}", &buf[..amt]);
        Message::parse(&mut buf[..amt]).pack();
    }
}
