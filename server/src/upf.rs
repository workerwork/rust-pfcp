use messages::Message;


pub fn run(buf: &mut [u8]) {
    Message::parse(buf).pack();
}
