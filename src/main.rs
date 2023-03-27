extern crate clap;

use clap::{App, Arg};
use std::fs;

fn main() {
    let matches = App::new("jsonlint")
        .version("1.0.0")
        .author("Philipp Speck <philipp@typo.media>")
        .about("A clone of jsonlint, written in Rust")
        .arg(Arg::new("file")
            .required(true)
            .help("JSON file to parse")
        )
        .arg(Arg::new("pretty")
            .short('p')
            .long("pretty")
            .takes_value(false)
            .help("Pretty prints a JSON file")
        )
        .get_matches();

    let path = matches.value_of("file").unwrap();
    let data = fs::read_to_string(path).expect("Error reading file");
    let pretty = matches.is_present("pretty");

    if pretty {
        match json_read(&data) {
            Ok(v) => println!("{}", json::stringify_pretty(v, 4)),
            Err(e) => println!("{}", e),
        }
    } else {
        match json_lint(&data) {
            Some(e) => println!("{}", e),
            _ => println!("{}", data)
        }
    }
}

fn json_read(json: &str) -> Result<json::JsonValue, json::JsonError> {
    json::parse(json)
}

fn json_lint(json: &str) -> Option<json::JsonError> {
    let result: Result<json::JsonValue, json::JsonError> = json::parse(json);
    result.err()
}