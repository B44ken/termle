mod game;
mod server;
mod answer;

use std::{collections::HashMap, net::TcpListener};

type TriesMap = HashMap<String, Vec<String>>;

fn main() {
    let mut ans = answer::get_random_word();
    let mut tries: TriesMap = HashMap::new();

    let listener = TcpListener::bind("0.0.0.0:4242").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        server::handle_connection(stream, &mut tries, ans);
    }
}