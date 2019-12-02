extern crate clap;

use std::process::exit;
use clap::{Arg, App, SubCommand};
mod day1;

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
            let file = matches.subcommand_matches("day1").unwrap().value_of("INPUT").unwrap();
            day1::main(file)
        },
        None => {
            println!("{}", matches.usage());
            Ok(())
        },
        _ => Err(format!("{}", "Unknown subcommand")),
    };

    match result {
        Ok(_) => exit(0),
        Err(e) => {
            eprintln!("ERROR: {}", e);
            exit(-1)
        }
    }
}