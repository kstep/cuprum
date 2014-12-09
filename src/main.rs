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

fn run_player(s: &mut Stream, qs: Option<TreeMap<String, String>>, mpc: &mut MpdConnection) -> IoResult<()> {
    debug!("params: {}", qs);

    s.write_str("Status: 200 OK\r\n").and_then(|()|
    s.write_str("Content-Type: application/json; charset=utf-8\r\n")).and_then(|()|
    s.write_str("\r\n")).and_then(|()| {
        if let Some(ref qs) = qs {
            if let Some(cmd) = qs.get(&"cmd".into_string()) {
                match cmd[] {
                    "next" => mpc.next(),
                    "prev" => mpc.prev(),
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
                        if let Some(volume) = qs.get(&"volume".into_string()).and_then(|v| from_str(v[])) {
                            mpc.set_volume(volume);
                        }
                        if let Some(elapsed_time) = qs.get(&"elapsed_time".into_string()).and_then(|v| from_str(v[])).map(|v| Duration::milliseconds(v)) {
                            mpc.current_song().and_then(|ref mut s| s.seek(mpc, elapsed_time));
                        }
                        if let Some(state) = qs.get(&"state".into_string()) {
                            match state[] {
                                "Play" => match qs.get(&"id".into_string()).and_then(|v| from_str(v[])) {
                                    Some(id) => mpc.play_id(id),
                                    None => mpc.play()
                                },
                                "Pause" => mpc.pause(true),
                                "Stop" => mpc.stop(),
                                _ => Ok(())
                            };
                        }
                        Ok(())
                    },
                    _ => Ok(())
                };
            }
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

fn run_outputs(s: &mut Stream, qs: Option<TreeMap<String, String>>, mpc: &mut MpdConnection) -> IoResult<()> {
    debug!("params: {}", qs);
    let mut outputs = mpc.outputs().unwrap();
    if let Some(ref qs) = qs {
        if let Some(enabled) = qs.get(&"enabled".into_string()).and_then(|v| from_str(v[])) {
            if let Some(id) = qs.get(&"id".into_string()).and_then(|v| from_str(v[])) {
                if let Some(Ok(mut output)) = outputs.find(|o| match *o {
                    Ok(ref v) => v.id() == id,
                    Err(_) => false
                }) {
                    debug!("enabled = {}, id = {}", enabled, id);
                    if enabled {
                        output.enable(mpc);
                    } else {
                        output.disable(mpc);
                    }
                    return json_result(s, json::encode(&output));
                }
            }
        }
    }
    json_result(s, json::encode(&outputs))
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
                        Some(b"player.json") => run_player(s, env.query(), &mut mpc),
                        Some(b"current-song.json") => json_result(s, json::encode(&mpc.current_song())),
                        Some(b"outputs.json") => run_outputs(s, env.query(), &mut mpc),
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
