use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

use crate::{answer::get_random_word, game::{create_termle, TermleResult}, TriesMap};

const MAX_TRIES: usize = 5;

#[derive(Clone)]
struct Headers {
    fingerprint: String,
    guess: String,
}

fn get_headers(buf: BufReader<&TcpStream>) -> Headers {
    let mut fingerprint = String::new();
    let mut guess = String::new();
    let fp_fields = vec![
        "Host",
        "User-Agent",
        "Accept",
        "Accept-Language",
        "Accept-Encoding",
        "Connection",
        "Upgrade-Insecure-Requests",
        "Cache-Control",
    ];

    for line in buf.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        let first_word = line.split_whitespace().next().unwrap();
        if first_word == "GET" {
            guess = line.split_whitespace().nth(1).unwrap().to_string();
            guess = guess.split_off(1).to_uppercase();
        } else if fp_fields.contains(&first_word) {
            fingerprint.push_str(&line);
            fingerprint.push('\n');
        }
    }

    Headers {
        fingerprint: fingerprint,
        guess: guess,
    }
}

fn append_tries(tries: &mut TriesMap, headers: Headers) -> i64 {
    if tries.contains_key(&headers.fingerprint) {
        tries
            .get_mut(&headers.fingerprint)
            .unwrap()
            .push(headers.guess);
    } else {
        tries.insert(headers.fingerprint.clone(), vec![headers.guess]);
    }

    tries.get(&headers.fingerprint).unwrap().len() as i64
}

pub fn build_response(term: TermleResult, tries: Vec<String>, answer: &str) -> String {
    let mut resp = String::new();
    resp.push_str("HTTP/1.1 200 OK\r\n\r\n");

    if tries.len() >= MAX_TRIES {
        resp.push_str("game over! the word was ");
        resp.push_str(answer);
        resp.push('\n');
        resp.push_str("your guesses: \n");
        resp.push_str(create_tries_block(&tries, answer).as_str());
    } else {
        resp.push_str(
            format!(
                "{} - {} tries left\n",
                term.to_ansi(),
                MAX_TRIES - tries.len()
            )
            .as_str(),
        );
    }

    resp
}

fn create_tries_block(tries: &Vec<String>, answer: &str) -> String {
    let mut resp = String::new();
    for guess in tries {
        resp.push_str(
            &create_termle(&guess, answer)
                .unwrap()
                .to_ansi_hidden()
                .as_str(),
        );
        resp.push('\n');
    }
    resp
}

pub fn handle_connection(mut stream: TcpStream, tries_map: &mut HashMap<String, Vec<String>>, mut answer: &str) {
    let buf = BufReader::new(&stream);
    let headers = get_headers(buf);
    if headers.guess == "api/reset" {
        stream.write_all("HTTP 1.1 200 OK\r\n\r\nresetting game".as_bytes()).unwrap();
        tries_map.clear();
        answer = get_random_word();
        return;
    }

    append_tries(tries_map, headers.clone());

    let term = create_termle(&headers.guess, answer);
    if term.is_none() {
        stream.write_all("HTTP 1.1 200 OK\r\n\r\ninvalid guess".as_bytes()).unwrap();
        return;
    }
    let response = build_response(
        term.unwrap(),
        tries_map.get(&headers.fingerprint).unwrap().clone(),
        answer
    );
    stream.write_all(response.as_bytes()).unwrap();
}
