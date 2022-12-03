use std::fs;
use std::str::Split;

pub fn day3_part1() {
    let input = fs::read_to_string("assets/day3.txt").expect("Input file missing");
    let lines = input.lines();
    let res: u32 =
        lines.map(|line| match line.split_at(line.len() / 2) {
            (left, right) => {
                left.chars().filter(|c| right.contains(*c)).fold(0, |r, c|
                    if c.is_ascii_lowercase() {
                        u32::from(c) - u32::from('a') + 1
                    } else {
                        u32::from(c) - u32::from('A') + 27
                    },
                )
            }
        }).sum();
    println!("Sum of rucksack priorities: {}", res);
}

