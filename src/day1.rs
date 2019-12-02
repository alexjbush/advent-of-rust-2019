use std::fs::File;
use std::io::{ prelude::*, BufReader};

pub fn main(filename: &str) -> std::result::Result<(), String> {
    let file = File::open(filename).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);


    let mut total = 0u32;
    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?.parse::<u32>().map_err(|e| e.to_string())?;
        total += mass_to_fuel(line);
    }

    println!("Sum of fuel requirements: {}", total);

    Ok(())
}

//Fuel required to launch a given module is based on its mass. Specifically, to find the fuel required for a module, take its mass, divide by three, round down, and subtract 2.
fn mass_to_fuel(mass: u32) -> u32 {
    (mass as f32 / 3.0f32).floor() as u32 - 2
}