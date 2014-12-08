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
    s.write_str("Content-Type: application/json; charset=utf-8\r\n")).and_then(|()|
    s.write_str("\r\n")).and_then(|()|
    s.write_str("{\"error\":\"object not found\"}"))
}

fn json_result(s: &mut Stream, r: String) -> IoResult<()> {
    s.write_str("Status: 200 OK\r\n").and_then(|()|
    s.write_str("Content-Type: application/json; charset=utf-8\r\n")).and_then(|()|
    s.write_str("\r\n")).and_then(|()|
    s.write_str(r[]))
}

fn run(s: &mut Stream, env: &SCGIEnv) -> IoResult<()> {

    let mut mpc = MpdConnection::new(None, 6600).unwrap().unwrap();

    match env.path("DOCUMENT_URI") {
        None => not_found(s),
        Some(path) => {
            let mut parts = path.components();
            match parts.next() {
                Some(b"plugins") => match parts.next() {
                    Some(b"mpd") => match parts.next() {
                        Some(b"queue.json") => json_result(s, json::encode(&mpc.queue().songs())),
                        Some(b"status.json") => json_result(s, json::encode(&mpc.status())),
                        Some(b"current-song.json") => json_result(s, json::encode(&mpc.current_song())),
                        Some(b"outputs.json") => json_result(s, json::encode(&mpc.outputs())),
                        Some(b"playlists.json") => json_result(s, json::encode(&mpc.playlists())),
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
