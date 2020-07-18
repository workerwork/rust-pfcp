mod args;
mod web;

use messages::Message;
use std::net::UdpSocket;
//use std::process::exit;
//use std::thread;
//use web::web_inf::http_server;
//use std::env;

fn main() {
    //web interface handler thread
    //thread::spawn(http_server);
    let (socket,) = args::get_args();
    let mut buf = [0u8; 65535];
    loop {
        let (amt, src) = socket.recv_from(&mut buf).unwrap();
        println!("received {} bytes from: {:?}", amt, src);
        println!("{:?}", &buf[..amt]);
        Message::parse(&mut buf[..amt]).pack();
    }
}
