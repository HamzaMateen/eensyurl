extern crate base64;
use base64::engine::general_purpose;
use base64::Engine;
use std::env;
use std::{collections::HashMap, hash::Hash};
use ulid::Ulid;

use {once_cell::sync::Lazy, regex::Regex};

// 2. helper function for part 2
fn is_valid_url(potential_url: &str) -> bool {
    const URL_REGEX: &str = r"^(https?://)?([\w-]+\.)+[\w-]+(/[\w\- ./?%&=]*)?$";
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(URL_REGEX).unwrap());
    RE.is_match(potential_url)
}

fn generate_shortened_url() -> String {
    const SHORTENED_URL_PREFIX: &str = "https://eensy.url/";

    let uri_ulid = Ulid::new();

    let mut shortened_url = String::from(SHORTENED_URL_PREFIX);
    shortened_url.push_str(uri_ulid.to_string().as_str());

    shortened_url
}

fn main() {
    // 1. take a url from the command line
    let args: Vec<String> = env::args().collect();

    let program_name = &args[0];
    let potential_long_url = args.get(1);

    let long_url = match potential_long_url {
        Some(long_url) => long_url,
        None => {
            println!("Usage: {program_name} <long_url>");
            std::process::exit(1);
        }
    };

    // 2. parse it via regex to see if it is a valid URL
    if is_valid_url(long_url.as_str()) {
        println!("Yeah");
    } else {
        println!("nahh");
    }

    // 3. ... wait for next steps
    // so how do you shorten a URL?
    // let me go very simple
    let short_url = generate_shortened_url().to_lowercase();

    // 4. convert to base 64 using Engine
    let b64 = general_purpose::STANDARD.encode(&short_url);
    println!("your eensy url is: {}", b64);
}
