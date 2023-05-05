extern crate afl;
extern crate rustls;

use rustls::internal::msgs::codec::{Codec, Reader};
use rustls::internal::msgs::persist;
use std::io;
use std::io::Read;

fn try_type<T>(data: &[u8])
where
    T: Codec,
{
    let mut rdr = Reader::init(data);

    let _ = T::read(&mut rdr);
}


fn main(){
    let mut input = vec![];
    let data = io::stdin().read_to_end(&mut input);

  try_type::<persist::ServerSessionValue>(&input);

}
