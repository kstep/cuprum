#[macro_use] extern crate nickel;
#[macro_use] extern crate log;

extern crate rustc_serialize;
extern crate mpd;

use rustc_serialize::json::encode;

use nickel::{Nickel, MediaType, JsonBody};

#[derive(Debug, RustcDecodable)]
struct QueuePlaceRequest {
    id: Option<mpd::Id>,
    pos: Option<u32>,
}

#[derive(Debug, RustcDecodable)]
struct MpdPatchStatusRequest {
    volume: Option<i8>,
    repeat: Option<bool>,
    random: Option<bool>,
    single: Option<bool>,
    consume: Option<bool>,
    state: Option<mpd::State>,
    crossfade: Option<i64>,
    mixrampdb: Option<f32>,
    mixrampdelay: Option<i64>,
    replaygain: Option<mpd::ReplayGain>,
    song: Option<QueuePlaceRequest>,
}

#[derive(Debug, RustcDecodable)]
enum MpdPlaybackRequest {
    Next,
    Prev,
}

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "/mpd/status" => |_req, mut res| {
            res.set(MediaType::Json);
            encode(&mpd::Client::default().status().unwrap()).unwrap().to_string()
        }
        patch "/mpd/status" => |req, _res| {
            let body = req.json_as::<MpdPatchStatusRequest>().unwrap();
            println!("status update: {:?}", body);
            let mut cli = mpd::Client::default();
            if let Some(v) = body.volume { cli.volume(v); }
            if let Some(v) = body.repeat { cli.repeat(v); }
            if let Some(v) = body.single { cli.single(v); }
            if let Some(v) = body.random { cli.random(v); }
            if let Some(v) = body.consume { cli.consume(v); }
            if let Some(v) = body.crossfade { cli.crossfade(v); }
            if let Some(v) = body.mixrampdb { cli.mixrampdb(v); }
            if let Some(v) = body.mixrampdelay { cli.mixrampdelay(v); }
            if let Some(v) = body.replaygain { cli.replaygain(v); }
            if let Some(v) = body.song {
                if let Some(p) = v.pos { cli.switch(p).unwrap(); }
                if let Some(p) = v.id { cli.switch(p).unwrap(); }
            }
            if let Some(v) = body.state {
                match v {
                    mpd::State::Stop => { cli.stop(); },
                    mpd::State::Play => { cli.play(); },
                    mpd::State::Pause => { cli.pause(true); },
                }
            }
            ""
        }
        get "/mpd/queue" => |_req, mut res| {
            res.set(MediaType::Json);
            encode(&mpd::Client::default().queue().unwrap()).unwrap().to_string()
        }
        get "/mpd/stats" => |_req, mut res| {
            res.set(MediaType::Json);
            encode(&mpd::Client::default().stats().unwrap()).unwrap().to_string()
        }
        get "/mpd/song" => |_req, mut res| {
            res.set(MediaType::Json);
            encode(&mpd::Client::default().currentsong().unwrap()).unwrap().to_string()
        }
        post "/mpd/playback" => |req, _res| {
            let body = req.json_as::<MpdPlaybackRequest>().unwrap();
            let mut cli = mpd::Client::default();
            match body {
                MpdPlaybackRequest::Next => { cli.next().unwrap(); },
                MpdPlaybackRequest::Prev => { cli.prev().unwrap(); },
            }
        }
    });

    server.listen("127.0.0.1:7777");
}
