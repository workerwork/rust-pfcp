//#[macro_use]
//extern crate dotenv_codegen;
mod args;
//mod web;
mod log;
mod redis;
mod upf;
use upf::core;

//use messages::Message;
use std::thread;
//use std::sync::mpsc;
//use web::web_inf;

fn main() {
    //web interface handler thread
    //thread::spawn(web_inf::http_server);

    let t = redis::get().unwrap();
    println!("{}", t);

    log::logger();

    core::run();
}
