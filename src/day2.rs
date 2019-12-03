use either::Either::{self, Left, Right};
use std::fs;

pub fn main(input: Either<&str, &str>, replacements: Vec<(usize, usize)>, part2: bool) -> std::result::Result<(), String> {
    let program = match input {
        Left(f) => fs::read_to_string(f).map_err(|e| e.to_string())?,
        Right(i) => i.to_string(),
    };

    let mut program = program_string_to_vect(program)?;

    if !part2 {
        run(&mut program, replacements)?;
        let output: String = program.into_iter().map(|i| i.to_string()).collect::<Vec<String>>().join(",");
        println!("{}", output);
        Ok(())
    } else {
        for a in 0usize..=99 {
            for b in 0usize..=99 {
                let mut input = program.clone();
                let res = run(&mut input, vec![(1, a), (2, b)]);
                if res.is_ok() && input.get(0).map(|v| v.clone() == 19690720usize).unwrap_or(false) {
                    println!("{}", 100 * a + b);
                    return Ok(())
                }
            }
        }
        Err("Did not finish".to_string())
    }
}

pub fn run(program: &mut Vec<usize>, replacements: Vec<(usize, usize)>) -> std::result::Result<(), String> {

    let mut program = program;

    replacements.into_iter().for_each(|(a, b)| {
        let v = program.get_mut(a).unwrap();
        *v = b
    });

    run_program(&mut program, 0)?;

    Ok(())
}

fn program_string_to_vect(input: String) -> std::result::Result<Vec<usize>, String> {
    input
        .split(",")
        .map(|x| x.parse::<usize>())
        .collect::<Result<Vec<_>, std::num::ParseIntError>>()
        .map_err(|e| e.to_string())
}

fn run_program(program: &mut Vec<usize>, curr_position: usize) -> std::result::Result<(), String> {
    let code = program.get(curr_position).ok_or("Reached end of program without encountering opt-code 99")?.clone();
    match code {
        99 => Ok(()),
        i if i == 1 || i == 2 => {
            let input1 = program.get(curr_position + 1).and_then(|&a| program.get(a)).ok_or("Failed to get input1")?.clone();
            let input2 = program.get(curr_position + 2).and_then(|&b| program.get(b)).ok_or("Failed to get input2")?.clone();
            let output = program.get(curr_position + 3).map(|c| c.clone()).and_then(|c| program.get_mut(c)).ok_or("Failed to get input3")?;
            if i == 1 {
                *output = input1 + input2
            } else {
                *output = input1 * input2
            }
            run_program(program, curr_position + 4)
        },
        i => Err(format!("Unrecognised op-code: {}", i)),
    }
}