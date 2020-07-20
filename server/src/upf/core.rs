use super::super::args;
//use super::queue::Queue;
use messages::Message;
use std::sync::mpsc;
use std::thread;
use std::time;

pub fn run() {
    let (socket,) = args::get_args();
    let mut buf = [0u8; 65535];

    loop {
        let (amt, src) = socket.recv_from(&mut buf).unwrap();
        println!("received {} bytes from: {:?}", amt, src);
        println!("{:?}", &buf[..amt]);
        //Message::parse(&mut buf[..amt]).pack();

        let (tx, rx) = mpsc::channel();
        let message = Message::parse(&mut buf[..amt]);
        tx.send(message).unwrap();

        thread::spawn(move || {
            let message = rx.recv().unwrap();
            println!("Got: {:?}", message);
            message.pack();
            let ten_millis = time::Duration::from_millis(10000);
            thread::sleep(ten_millis);
            //todo ...
            println!("ok");
        });
        //.join()
        //.unwrap();
        /*let mut q = Queue::new();
        q.push(message);
        //println!("{:?}", q);
        if let Some(message) = q.pop() {
            //todo something!
            //多线程
            message.pack();
        }*/
        println!("test");
    }
    println!("over");
}
