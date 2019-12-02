use either::Either::{self, Left, Right};

pub fn main(input: Either<&str, &str>) -> std::result::Result<(), String> {
    let msg = match input {
        Left(f) => format!("File: {}", f),
        Right(i) => format!("Input: {}", i),
    };

    println!("{}", msg);

    Ok(())
}