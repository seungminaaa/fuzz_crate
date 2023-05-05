
extern crate afl;
extern crate rustls;

use rustls::internal::msgs::deframer;
use rustls::internal::msgs::message::Message;
use rustls::internal::record_layer::RecordLayer;
use std::io;
use std::io::Read;

fn main() {
    let mut input = vec![];
    let data = io::stdin().read_to_end(&mut input);

    let mut dfm = deframer::MessageDeframer::default();
    if dfm
        .read(&mut io::Cursor::new(&input))
        .is_err()
    {
        return;
    }
    dfm.has_pending();

    let mut rl = RecordLayer::new();
    while let Ok(Some(decrypted)) = dfm.pop(&mut rl) {
        Message::try_from(decrypted.message).ok();
    }

}
