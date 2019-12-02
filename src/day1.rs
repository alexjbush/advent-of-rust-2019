use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn main(filename: &str, is_part2: bool) -> std::result::Result<(), String> {
    let file = File::open(filename).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);


    let mut total = 0i32;
    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?.parse::<i32>().map_err(|e| e.to_string())?;
        total += mass_to_fuel(line, is_part2);
    }

    println!("Sum of fuel requirements: {}, with fuel requiring mass: {}", total, is_part2);

    Ok(())
}

//Fuel required to launch a given module is based on its mass. Specifically, to find the fuel required for a module, take its mass, divide by three, round down, and subtract 2.
//Then, treat the fuel amount you just calculated as the input mass and repeat the process, continuing until a fuel requirement is zero or negative.
fn mass_to_fuel(mass: i32, fuel_requires_fuel: bool) -> i32 {
    let fuel = (mass as f32 / 3.0f32).floor() as i32 - 2;

    if fuel_requires_fuel && fuel > 0 {
        return fuel + mass_to_fuel(fuel, fuel_requires_fuel);
    } else {
        return fuel.max(0);
    }
}