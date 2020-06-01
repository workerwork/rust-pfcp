use elements::ie_type::IEType;
use messages::*;

fn main() {
    let s = 3;
    let mut h = Header::new();
    h.version = 2;
    h.seid = 100;
    //h.set_mp(true).set_s(true);
    let h = h.set_mp(true).set_s(true);
    println!("{:#?}", h);
    let t = h.seid;
    println!("{:#?}", h);
    match s {
        //S => println!("good!"),
        x if x == IEType::CreateFAR as i32 => println!("{:?}", x),
        //x @ 3 => println!("{:?}", s),
        _ => (),
    }
    //println!("{:?}", s);
    println!("Hello, world!");

    let msg = Message::ASR(association_setup_request::AssociationSetupRequest { header: h });
}
