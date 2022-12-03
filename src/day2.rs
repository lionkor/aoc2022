use std::fs;
use std::str::Split;

pub fn day2_part1() {
    let input = fs::read_to_string("assets/day2.txt").expect("Input file missing");
    let mut total = 0;
    for line in input.lines() {
        match line.split(' ').collect::<Vec<&str>>().as_slice() {
            ["A", "X"] => total += 1 + 3,
            ["A", "Y"] => total += 2 + 6,
            ["A", "Z"] => total += 3 + 0,
            ["B", "X"] => total += 1 + 0,
            ["B", "Y"] => total += 2 + 3,
            ["B", "Z"] => total += 3 + 6,
            ["C", "X"] => total += 1 + 6,
            ["C", "Y"] => total += 2 + 0,
            ["C", "Z"] => total += 3 + 3,
            _ => {}
        }
    }
    println!("Total score before the elf comes back: {}", total);
}

pub fn day2_part2() {
    let input = fs::read_to_string("assets/day2.txt").expect("Input file missing");
    let mut total = 0;
    for line in input.lines() {
        match line.split(' ').collect::<Vec<&str>>().as_slice() {
            ["A", "X"] => total += 3 + 0,
            ["A", "Y"] => total += 1 + 3,
            ["A", "Z"] => total += 2 + 6,
            ["B", "X"] => total += 1 + 0,
            ["B", "Y"] => total += 2 + 3,
            ["B", "Z"] => total += 3 + 6,
            ["C", "X"] => total += 2 + 0,
            ["C", "Y"] => total += 3 + 3,
            ["C", "Z"] => total += 1 + 6,
            _ => {}
        }
    }
    println!("Total score after the elf comes back: {}", total);
}