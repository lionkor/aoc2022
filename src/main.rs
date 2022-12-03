use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let input = fs::read_to_string("assets/day1.txt").expect("Input file missing");

    let mut most_calories_elf = -1;
    let mut most_calories = 0;
    let mut current_calories = 0;
    let mut current_elf = 1;

    for line in input.lines() {
        if line.trim().is_empty() {
            if current_calories > most_calories {
                most_calories = current_calories;
                most_calories_elf = current_elf;
            }
            current_elf += 1;
            current_calories = 0;
        } else {
            match u32::from_str(line) {
                Ok(calories) => {
                    current_calories += calories;
                }
                Err(err) => eprintln!("Error: '{}' isn't a valid u32: {}", line, err)
            }
        }
    }
    println!("Elf {} wins, with {} calories", most_calories_elf, most_calories);
}