extern crate afl;
extern crate rustls;

use rustls::internal::msgs::codec::Reader;
use rustls::internal::msgs::message::{Message, OpaqueMessage, PlainMessage};
use std::io::Read;
use std::io;
fn main(){
    let mut input = vec![];
    let data = io::stdin().read_to_end(&mut input);
    
    let mut rdr = Reader::init(&input);
    if let Ok(m) = OpaqueMessage::read(&mut rdr) {
        let msg = match Message::try_from(m.into_plain_message()) {
            Ok(msg) => msg,
            Err(_) => return,
        };
        //println!("msg = {:#?}", m);
        let enc = PlainMessage::from(msg)
            .into_unencrypted_opaque()
            .encode();
        //println!("data = {:?}", &data[..rdr.used()]);
        assert_eq!(enc, &input[..rdr.used()]);
    }

}
