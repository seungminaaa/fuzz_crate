extern crate afl;
extern crate rustls;
extern crate webpki;

use std::io::Read;
use rustls::{ClientConfig, ClientConnection, RootCertStore};
use std::io;
use std::sync::Arc;
fn main(){
    
    let mut input = vec![];
    let data = io::stdin().read_to_end(&mut input); 
     

    let root_store = RootCertStore::empty();
    let config = Arc::new(
        ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_store)
            .with_no_client_auth(),
    );
    let example_com = "example.com".try_into().unwrap();
    let mut client = ClientConnection::new(config, example_com).unwrap();
    let _ = client.read_tls(&mut io::Cursor::new(&input));
    let _ = client.process_new_packets();
}

