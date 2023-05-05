
extern crate afl;
extern crate rustls;

use rustls::server::ResolvesServerCert;
use rustls::{ServerConfig, ServerConnection};
use std::io::Read;
use std::io;
use std::sync::Arc;

struct Fail;

impl ResolvesServerCert for Fail {
    fn resolve(
        &self,
        _client_hello: rustls::server::ClientHello,
    ) -> Option<Arc<rustls::sign::CertifiedKey>> {
        None
    }
}
fn main(){
    let mut input = vec![];
    let data = io::stdin().read_to_end(&mut input);

    let config = Arc::new(
        ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_cert_resolver(Arc::new(Fail)),
    );
    let mut server = ServerConnection::new(config).unwrap();
    let _ = server.read_tls(&mut io::Cursor::new(&input));
    let _ = server.process_new_packets();
}
