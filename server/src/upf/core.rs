use super::super::args;
use super::queue::Queue;
use messages::Message;

pub fn run() {
    let (socket,) = args::get_args();
    let mut buf = [0u8; 65535];

    loop {
        let (amt, src) = socket.recv_from(&mut buf).unwrap();
        println!("received {} bytes from: {:?}", amt, src);
        println!("{:?}", &buf[..amt]);
        //Message::parse(&mut buf[..amt]).pack();

        let message = Message::parse(&mut buf[..amt]);
        let mut q = Queue::new();
        q.push(message);
        //println!("{:?}", q);
        if let Some(message) = q.pop() {
            //todo something!
            //多线程
            message.pack();
        }
    }
}
