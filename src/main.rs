mod answer;
mod game;
mod server;

use std::{collections::HashMap, net::TcpListener};

type TriesMap = HashMap<String, Vec<String>>;

fn main() {
    let mut ans = answer::get_random_word();
    let mut tries: TriesMap = HashMap::new();

    let listener = TcpListener::bind("0.0.0.0:4242").unwrap();
    for stream in listener.incoming() {
        server::handle_connection(stream.unwrap(), &mut tries, &mut ans);
    }
}
