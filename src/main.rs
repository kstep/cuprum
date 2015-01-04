#![feature(slicing_syntax, phase)]

extern crate serialize;
extern crate scgi;
extern crate mpd;
#[phase(plugin, link)] extern crate log;

use scgi::{TcpSCGIServer, SCGIEnv};
use std::io::{TcpStream, Stream, IoResult};
use mpd::client::MpdClient;
use serialize::json;
use std::str::from_utf8;
use std::collections::BTreeMap;
use std::time::duration::Duration;
use std::error::FromError;

fn not_found(s: &mut Stream) -> IoResult<()> {
    s.write_str("Status: 404 Not Found\r\n").and_then(|_|
    s.write_str("Content-Type: application/json; charset=utf-8\r\n")).and_then(|_|
    s.write_str("\r\n")).and_then(|_|
    s.write_str("{\"error\":\"object not found\"}"))
}

fn json_result(s: &mut Stream, r: String) -> IoResult<()> {
    s.write_str("Status: 200 OK\r\n").and_then(|_|
    s.write_str("Content-Type: application/json; charset=utf-8\r\n")).and_then(|_|
    s.write_str("\r\n")).and_then(|_|
    s.write_str(r[]))
}

type TcpMpdClient = MpdClient<TcpStream>;

fn run_player(s: &mut Stream, qs: Option<BTreeMap<String, String>>, mpc: &mut TcpMpdClient) -> IoResult<()> {
    debug!("params: {}", qs);

    s.write_str("Status: 200 OK\r\n").and_then(|_|
    s.write_str("Content-Type: application/json; charset=utf-8\r\n")).and_then(|_|
    s.write_str("\r\n")).and_then(|_| {
        if let Some(ref qs) = qs {
            if let Some(cmd) = qs.get("cmd") {
                match cmd[] {
                    "next" => mpc.next(),
                    "prev" => mpc.prev(),
                    "set" => {
                        if let Some(repeat) = qs.get("repeat").and_then(|v| v.parse()) {
                            mpc.set_repeat(repeat);
                        }
                        if let Some(single) = qs.get("single").and_then(|v| v.parse()) {
                            mpc.set_single(single);
                        }
                        if let Some(random) = qs.get("random").and_then(|v| v.parse()) {
                            mpc.set_random(random);
                        }
                        if let Some(consume) = qs.get("consume").and_then(|v| v.parse()) {
                            mpc.set_consume(consume);
                        }
                        if let Some(volume) = qs.get("volume").and_then(|v| v.parse()) {
                            mpc.set_volume(volume);
                        }
                        if let Some(elapsed_time) = qs.get("elapsed_time").and_then(|v| v.parse()).map(|v| Duration::milliseconds(v)) {
                            mpc.current_song().and_then(|ref mut s| s.seek(mpc, elapsed_time));
                        }
                        if let Some(state) = qs.get("state") {
                            match state[] {
                                "Play" => match qs.get("id").and_then(|v| v.parse()) {
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
    }).and_then(|_|
    s.write_str(json::encode(&mpc.status())[]))
}

fn run_queue(s: &mut Stream, method: String, qs: BTreeMap<String, String>, mpc: &mut TcpMpdClient) -> IoResult<()> {
    debug!("params: {}", qs);

    s.write_str("Status: 200 OK\r\n").and_then(|_|
    s.write_str("Content-Type: application/json; charset=utf-8\r\n")).and_then(|_|
    s.write_str("\r\n")).and_then(|_| {
        let mut queue = mpc.queue();
        match method[] {
            "GET" => s.write_str(json::encode(&queue.iter())[]),
            "DELETE" => if let Some(id) = qs.get("id").and_then(|v| v.parse()) {
                queue.remove_id(id);
                s.write_str("{}")
            } else {
                Ok(())
            },
            _ => Ok(())
        }
    })
}

fn run_outputs(s: &mut Stream, qs: Option<BTreeMap<String, String>>, mpc: &mut TcpMpdClient) -> IoResult<()> {
    debug!("params: {}", qs);
    if let Some(ref qs) = qs {
        if let Some(enabled) = qs.get("enabled").and_then(|v| v.parse()) {
            if let Some(id) = qs.get("id").and_then(|v| v.parse()) {
                if let Some(Ok(mut output)) = mpc.outputs().find(|o| match *o {
                    Ok(ref v) => v.id == id,
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
    json_result(s, json::encode(&mpc.outputs()))
}

fn run(s: &mut Stream, env: &SCGIEnv) -> IoResult<()> {

    let mut mpc = TcpStream::connect("localhost:6600").map_err(FromError::from_error).and_then(|c| MpdClient::new(c)).unwrap();

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
    let server = TcpSCGIServer::new("localhost:9000").unwrap();
    server.run(run);
}
