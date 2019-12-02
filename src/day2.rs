use either::Either::{self, Left, Right};
use std::fs;
use std::error::Error;

pub fn main(input: Either<&str, &str>) -> std::result::Result<(), String> {
    let raw_program = match input {
        Left(f) => fs::read_to_string(f).map_err(|e| e.description())?,
        Right(i) => i.into_string(),
    };

    println!("{}", raw_program);

    Ok(())
}