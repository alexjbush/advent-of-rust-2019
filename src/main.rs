extern crate clap;

use std::process::exit;
use clap::{Arg, App, SubCommand};
use std::error::Error;
use advent-of-rust;
fn main() {
    let matches = App::new("Advent of Rust")
        .version("0.1")
        .author("Alex Bush <bushnoh@gmail.com>")
        .subcommand(SubCommand::with_name("day1")
            .arg(Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1)))
        .get_matches();

    let result = match matches.subcommand_name() {
        Some("day1") => {
            let file = matches.subcommand_matches("day1").unwrap().value_of("INPUT");
            day1(file)
        },
        None => Err(std::io::Error::new(std::io::ErrorKind::Other, "No subcommand given")),
        _ => Err(std::io::Error::new(std::io::ErrorKind::Other, "Unknown subcommand")),
    };

    match result {
        Ok(_) => exit(0),
        Err(e) => {
            eprint!("{}", e.description());
            exit(-1)
        }
    }
}