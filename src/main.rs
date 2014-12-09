#![feature(slicing_syntax, phase)]

extern crate serialize;
extern crate scgi;
extern crate mpd;
#[phase(plugin, link)] extern crate log;

use scgi::{SCGIBind, TcpSCGIServer, SCGIEnv};
use std::io::{Stream, IoResult};
use mpd::connection::MpdConnection;
use serialize::json;
use std::str::from_utf8;
use std::collections::TreeMap;
use std::time::duration::Duration;

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

fn run_player(s: &mut Stream, qs: TreeMap<String, String>, mpc: &mut MpdConnection) -> IoResult<()> {
    debug!("params: {}", qs);

    s.write_str("Status: 200 OK\r\n").and_then(|()|
    s.write_str("Content-Type: application/json; charset=utf-8\r\n")).and_then(|()|
    s.write_str("\r\n")).and_then(|()| {
        if let Some(cmd) = qs.get(&"cmd".into_string()) {
            match cmd[] {
                "play" => match qs.get(&"id".into_string()).and_then(|v| from_str(v[])) {
                    Some(id) => mpc.play_id(id),
                    None => mpc.play()
                },
                "stop" => mpc.stop(),
                "pause" => mpc.pause(true),
                "next" => mpc.next(),
                "prev" => mpc.prev(),
                "seek" => mpc.current_song()
                    .and_then(|ref mut s| s.seek(mpc,
                                                 qs.get(&"elapsed_time".into_string())
                                                 .and_then(|v| from_str(v[]))
                                                 .map(|v| Duration::milliseconds(v))
                                                 .unwrap_or(Duration::zero()))),
                "set" => {
                    if let Some(repeat) = qs.get(&"repeat".into_string()).and_then(|v| from_str(v[])) {
                        mpc.set_repeat(repeat);
                    }
                    if let Some(single) = qs.get(&"single".into_string()).and_then(|v| from_str(v[])) {
                        mpc.set_single(single);
                    }
                    if let Some(random) = qs.get(&"random".into_string()).and_then(|v| from_str(v[])) {
                        mpc.set_random(random);
                    }
                    if let Some(consume) = qs.get(&"consume".into_string()).and_then(|v| from_str(v[])) {
                        mpc.set_consume(consume);
                    }
                    Ok(())
                },
                _ => Ok(())
            };
        }
        Ok(())
    }).and_then(|()|
    s.write_str(json::encode(&mpc.status())[]))
}

fn run_queue(s: &mut Stream, method: String, qs: TreeMap<String, String>, mpc: &mut MpdConnection) -> IoResult<()> {
    debug!("params: {}", qs);

    s.write_str("Status: 200 OK\r\n").and_then(|()|
    s.write_str("Content-Type: application/json; charset=utf-8\r\n")).and_then(|()|
    s.write_str("\r\n")).and_then(|()| {
        let mut queue = mpc.queue();
        match method[] {
            "GET" => s.write_str(json::encode(&queue.iter())[]),
            "DELETE" => if let Some(id) = qs.get(&"id".into_string()).and_then(|v| from_str(v[])) {
                queue.remove_id(id);
                s.write_str("{}")
            } else {
                Ok(())
            },
            _ => Ok(())
        }
    })
}

fn run(s: &mut Stream, env: &SCGIEnv) -> IoResult<()> {

    let mut mpc = MpdConnection::new(None, 6600).unwrap().unwrap();

    match env.path("DOCUMENT_URI") {
        None => not_found(s),
        Some(path) => {
            debug!("processing {}", path.display());
            let mut parts = path.components();
            match parts.next() {
                Some(b"plugins") => match parts.next() {
                    Some(b"mpd") => match parts.next() {
                        Some(b"queue.json") => run_queue(s, env.get("REQUEST_METHOD").unwrap(), env.query().unwrap(), &mut mpc),
                        Some(b"player.json") => run_player(s, env.query().unwrap(), &mut mpc),
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
