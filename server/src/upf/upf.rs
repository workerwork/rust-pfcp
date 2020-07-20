use super::queue::Queue;
use messages::Message;

pub fn run(buf: &mut [u8]) {
    //Message::parse(buf).pack();
    let message = Message::parse(buf);
    let mut q = Queue::new();
    q.push(message);
    //println!("{:?}", q);
    if let Some(message) = q.pop() {
        //todo something!
        //多线程
        message.pack();
    }
}
