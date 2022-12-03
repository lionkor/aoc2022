use std::cmp::max;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

pub fn day1_part1() {
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

pub fn day1_part2() {
    let input = fs::read_to_string("assets/day1.txt").expect("Input file missing");

    let mut calories_per_elf: Vec<u32> = vec![0];
    let mut current_elf = 1;

    for line in input.lines() {
        if line.trim().is_empty() {
            current_elf += 1;
            calories_per_elf.push(0);
        } else {
            match u32::from_str(line) {
                Ok(calories) => {
                    calories_per_elf[current_elf - 1] += calories;
                }
                Err(err) => eprintln!("Error: '{}' isn't a valid u32: {}", line, err)
            }
        }
    }
    calories_per_elf.sort();
    calories_per_elf.reverse();
    let top_three_sum = calories_per_elf[0..3].iter().fold(0, |a, b| a + b);
    println!("Sum of calories of the top 3 elves: {:?}", top_three_sum);
}