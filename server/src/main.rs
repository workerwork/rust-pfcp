//#[macro_use]
//extern crate dotenv_codegen;
mod args;
//mod web;
mod redis;

use messages::Message;
use std::thread;
//use std::sync::mpsc;
//use web::web_inf;

fn main() {
    //web interface handler thread
    //thread::spawn(web_inf::http_server);

    let (socket,) = args::get_args();
    let mut buf = [0u8; 65535];

    let t = redis::get().unwrap();
    println!("{}", t);

    loop {
        let (amt, src) = socket.recv_from(&mut buf).unwrap();
        println!("received {} bytes from: {:?}", amt, src);
        println!("{:?}", &buf[..amt]);
        Message::parse(&mut buf[..amt]).pack();
    }
}
