use either::Either::{self, Left, Right};
use std::fs;

pub fn main(input: Either<&str, &str>) -> std::result::Result<(), String> {
    let program = match input {
        Left(f) => fs::read_to_string(f).map_err(|e| e.to_string())?,
        Right(i) => i.to_string(),
    };

    let program = program_string_to_vect(program)?;


    Ok(())
}

fn program_string_to_vect(input: String) -> std::result::Result<Vec<i32>, String> {
    input
        .split(",")
        .map(|x| x.parse::<i32>())
        .collect::<Result<Vec<_>, std::num::ParseIntError>>()
        .map_err(|e| e.to_string())
}

fn run_program(program: Vec<i32>, curr_position: usize, to_step: usize) -> std::result::Result<Vec<i32>, String> {
    match program.get(curr_position) {
        None => Err("Reached end of program without encountering opt-code 99".to_string()),
        Some(99) => Ok(program),
        Some(_) if to_step > 0 => run_program(program, curr_position + 1, to_step - 1),
        Some(i) if i == &1i32 || i == &2i32 => {
            let f = if i == &1i32 { |a, b| a + b } else { |a, b| a * b };
            let c = calculate_result(program.get(curr_position +1), program.get(curr_position +2), f)?;
            let c_index = match program.get(curr_position + 3) {
                None => Err("Program tried to access an unavailable address".to_string()),
                Some(c_index) => c_index
            }?;

            Ok(program)
        },
        Some(i) => Err(format!("Unrecognised op-code: {}", i)),
    }
}

fn calculate_result(a: Option<&i32>, b: Option<&i32>, f: fn(&i32, &i32) -> i32) -> std::result::Result<i32, String> {
    match (a, b) {
        (None, _) | (_, None) => Err("Program tried to access an unavailable address".to_string()),
        (Some(a), Some(b)) => Ok(f(a, b))
    }
}