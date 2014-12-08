#![feature(slicing_syntax)]

extern crate serialize;
extern crate scgi;
extern crate mpd;

use scgi::{SCGIBind, TcpSCGIServer, SCGIEnv};
use std::io::{Stream, IoResult};
use mpd::connection::MpdConnection;
use serialize::json;
use std::str::from_utf8;

fn not_found(s: &mut Stream) -> IoResult<()> {
    s.write_str("Status: 404 Not Found\r\n").and_then(|()|
    s.write_str("\r\n")).and_then(|()|
    s.write_str("<h1>404: Object not found</h1>\r\n"))
}

fn json_result(s: &mut Stream, r: String) -> IoResult<()> {
    s.write_str("Status: 200 OK\r\n").and_then(|()|
    s.write_str("\r\n")).and_then(|()|
    s.write_str(r[]))
}

fn run(s: &mut Stream, env: &SCGIEnv) -> IoResult<()> {

    let mut mpc = MpdConnection::new(None, 6600).unwrap().unwrap();

    match env.path("DOCUMENT_URI") {
        None => not_found(s),
        Some(path) => {
            let parts : Vec<String> = path.components().map(|v| String::from_utf8_lossy(v).to_string()).collect();
            match parts[0][] {
                "plugins" => match parts[1][] {
                    "mpd" => match parts[2][] {
                        "queue.json" => json_result(s, json::encode(&mpc.queue().songs())),
                        "status.json" => json_result(s, json::encode(&mpc.status())),
                        "current-song.json" => json_result(s, json::encode(&mpc.current_song())),
                        "outputs.json" => json_result(s, json::encode(&mpc.outputs())),
                        "playlists.json" => json_result(s, json::encode(&mpc.playlists())),
                        _ => not_found(s)
                    },
                    _ => not_found(s)
                },

                _ => not_found(s)
            }
        }
    }
}

fn main() {
    let server : TcpSCGIServer = SCGIBind::new("localhost:9000").unwrap();
    server.run(run);
}
