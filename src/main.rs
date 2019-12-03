extern crate clap;
extern crate either;

use std::process::exit;
use clap::{Arg, App, SubCommand};
use either::Either::{Left, Right};

mod day1;
mod day2;

fn main() {
    let matches = App::new("Advent of Rust")
        .version("0.1")
        .author("Alex Bush <bushnoh@gmail.com>")
        .subcommand(SubCommand::with_name("day1")
            .arg(Arg::with_name("part2")
                .help("Enable output for part 2")
                .long("part2"))
            .arg(Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1)))
        .subcommand(SubCommand::with_name("day2")
            .arg(Arg::with_name("replace")
                .help("Initial replacements")
                .takes_value(true)
                .short("r")
                .long("replace")
                .multiple(true))
            .arg(Arg::with_name("part2")
                .help("Enable output for part 2")
                .long("part2"))
            .arg(Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .index(1)
                .required(true)
                .conflicts_with("input-string"))
            .arg(Arg::with_name("input-string")
                .help("Input from command-line")
                .long("input-string")
                .short("i")
                .required(true)
                .takes_value(true)
                .conflicts_with("INPUT")))
        .get_matches();

    let result = match matches.subcommand_name() {
        Some("day1") => {
            let sub = matches.subcommand_matches("day1").unwrap();
            let file = sub.value_of("INPUT").unwrap();
            day1::main(file, sub.is_present("part2"))
        }
        Some("day2") => {
            let sub = matches.subcommand_matches("day2").unwrap();
            let replacements = sub.values_of("replace").map(|x| x.into_iter().collect::<Vec<&str>>()).unwrap_or(Vec::new());
            let replacements: Vec<Vec<usize>> = replacements.into_iter().map(|s| s.split(",").map(|i| i.parse::<usize>().unwrap()).collect()).collect();
            let replacements = replacements.into_iter().map(
                |x| match (x.get(0), x.get(1), x.get(2)) {
                    (Some(k), Some(v), None) => (k.clone(), v.clone()),
                        _ => panic!("Replacement invalid")
                }
            ).collect::<Vec<(usize, usize)>>();
            let input = match (sub.value_of("INPUT"), sub.value_of("input-string")) {
                (Some(f), _) => Left(f),
                (_, Some(i)) => Right(i),
                _ => unreachable!()
            };
            day2::main(input, replacements, sub.is_present("part2"))
        }
        None => {
            println!("{}", matches.usage());
            Ok(())
        }
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