use either::Either::{self, Left, Right};
use std::fs;
use std::error::Error;

pub fn main(input: Either<&str, &str>) -> std::result::Result<(), String> {
    let program: &str = match input {
        Left(f) => fs::read_to_string(f).map_err(|e| e.description())?.as_str(),
        Right(i) => i,
    };

    println!("{}", program);

    Ok(())
}

fn program_string_to_vect(input: &str) -> Vec<i32> {
    let elems = input.split(",").map(|x| x.parse::<i32>());
    elems
}