//mod queue;
use super::queue::Queue;
use messages::Message;


pub fn run(buf: &mut [u8]) {
    //Message::parse(buf).pack();
    println!("comeon");
    let message = Message::parse(buf);
    let mut q = Queue::new();
    q.push(1);
    q.push(2);
    println!("{:?}", q);
    q.pop();
    println!("{:?}", q);

    q.pop();
    println!("Hello, world!");

    q.pop();
    println!("Hello, world!");
}
