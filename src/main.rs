#![feature(slicing_syntax, phase)]
#![feature(old_orphan_check)]

extern crate "rustc-serialize" as rustc_serialize;
extern crate scgi;
extern crate mpd;
#[phase(plugin, link)] extern crate log;

use scgi::{SCGIServer, SCGIEnv};
use std::io::{TcpStream, TcpListener, Stream, IoResult};
use std::io::{File, BufferedReader};
use std::path::Path;
use mpd::client::MpdClient;
use rustc_serialize::json;
use std::collections::BTreeMap;
use std::time::duration::Duration;
use std::error::FromError;
use std::ascii::AsciiExt;

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
                            mpc.repeat(repeat);
                        }
                        if let Some(single) = qs.get("single").and_then(|v| v.parse()) {
                            mpc.single(single);
                        }
                        if let Some(random) = qs.get("random").and_then(|v| v.parse()) {
                            mpc.random(random);
                        }
                        if let Some(consume) = qs.get("consume").and_then(|v| v.parse()) {
                            mpc.consume(consume);
                        }
                        if let Some(volume) = qs.get("volume").and_then(|v| v.parse()) {
                            mpc.volume(volume);
                        }
                        if let Some(elapsed_time) = qs.get("elapsed_time").and_then(|v| v.parse()).map(|v| Duration::milliseconds(v)) {
                            mpc.current_song().and_then(|ref mut s| s.seek(mpc, elapsed_time));
                        }
                        if let Some(state) = qs.get("state") {
                            match state.to_ascii_lowercase()[] {
                                "play" => match qs.get("id").and_then(|v| v.parse()) {
                                    Some(id) => mpc.play_id(id),
                                    None => mpc.play()
                                },
                                "pause" => mpc.pause(true),
                                "stop" => mpc.stop(),
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

fn run_queue(s: &mut Stream, method: String, qs: Option<BTreeMap<String, String>>, mpc: &mut TcpMpdClient) -> IoResult<()> {
    debug!("params: {}", qs);

    s.write_str("Status: 200 OK\r\n").and_then(|_|
    s.write_str("Content-Type: application/json; charset=utf-8\r\n")).and_then(|_|
    s.write_str("\r\n")).and_then(|_| {
        match method[] {
            "GET" => {
                if let Some(ref qs) = qs {
                    if let Some(name) = qs.get("name") {
                        mpc.clear().and_then(|_| mpc.load(name[]));
                    }
                }
            },
            "DELETE" => {
                if let Some(ref qs) = qs {
                    if let Some(id) = qs.get("id").and_then(|v| v.parse::<uint>()) {
                        //queue.remove_id(id);
                    }
                }
            },
            _ => ()
        }
        s.write_str(json::encode(&mpc.queue())[])
    })
}

fn run_outputs(s: &mut Stream, qs: Option<BTreeMap<String, String>>, mpc: &mut TcpMpdClient) -> IoResult<()> {
    debug!("params: {}", qs);
    if let Some(ref qs) = qs {
        if let Some(enabled) = qs.get("enabled").and_then(|v| v.parse()) {
            if let Some(id) = qs.get("id").and_then(|v| v.parse()) {
                if let Ok(Some(mut output)) = mpc.outputs().map(|os| os.into_iter().find(|o| o.id == id)) {
                    debug!("enabled = {}, id = {}", enabled, id);
                    if enabled {
                        output.enable(mpc).unwrap();
                    } else {
                        output.disable(mpc).unwrap();
                    }
                    return json_result(s, json::encode(&output));
                }
            }
        }
    }
    json_result(s, json::encode(&mpc.outputs()))
}

fn run(s: &mut Stream, env: &SCGIEnv) -> IoResult<()> {

    let mut mpc = TcpStream::connect("192.168.1.10:6600").map_err(FromError::from_error).and_then(|c| MpdClient::new(c)).unwrap();

    match env.path("DOCUMENT_URI") {
        None => not_found(s),
        Some(path) => {
            debug!("processing {}", path.display());
            let mut parts = path.components();
            match parts.next() {
                Some(b"plugins") => match parts.next() {
                    Some(b"mpd") => match parts.next() {
                        Some(b"queue.json") => run_queue(s, env.method(), env.query(), &mut mpc),
                        Some(b"player.json") => run_player(s, env.query(), &mut mpc),
                        Some(b"current-song.json") => json_result(s, json::encode(&mpc.current_song())),
                        Some(b"outputs.json") => run_outputs(s, env.query(), &mut mpc),
                        Some(b"playlists.json") => json_result(s, json::encode(&mpc.playlists())),
                        _ => not_found(s)
                    },
                    Some(b"dnsmasq") => match parts.next() {
                        Some(b"leases.json") => {
                            let mut leases_file = BufferedReader::new(File::open(&Path::new("/var/lib/misc/dnsmasq.leases")).unwrap());

                            #[derive(RustcEncodable)]
                            struct DnsmasqLease {
                                expires: u64,
                                macaddr: String,
                                ipaddr: String,
                                hostname: String,
                                hostid: String
                            };

                            let leases = leases_file.lines().flat_map(|line| line.ok().into_iter()).map(|line| {
                                let mut rec = line.trim().splitn(4, ' ');
                                DnsmasqLease {
                                    expires: rec.next().unwrap().parse().unwrap(),
                                    macaddr: rec.next().unwrap().to_string(),
                                    ipaddr: rec.next().unwrap().to_string(),
                                    hostname: rec.next().unwrap().to_string(),
                                    hostid: rec.next().unwrap().to_string(),
                                }
                            }).collect::<Vec<DnsmasqLease>>();
                            json_result(s, json::encode(&leases))
                        },
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
    let server = SCGIServer::new(TcpListener::bind("0.0.0.0:9000").unwrap());
    server.run(run);
}
