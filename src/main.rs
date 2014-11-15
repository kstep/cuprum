extern crate scgi;

use scgi::{SCGIBind, TcpSCGIServer, SCGIEnv};
use std::io::{Stream, IoResult};

fn run(s: &mut Stream, env: &SCGIEnv) -> IoResult<()> {
    try!(s.write_str("Status: 200 OK\r\n"));
    try!(s.write_str("\r\n"));
    Ok(())
}

fn main() {
    let server : TcpSCGIServer = SCGIBind::new("localhost:9000").unwrap();
    server.run(run);
}
