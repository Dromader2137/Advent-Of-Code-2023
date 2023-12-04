use std::{io::{Error, Read}, fs::File};
use num::pow;

fn main() -> Result<(), Error> {
    let mut input = String::new();
    File::open("test.in")
        .expect("Failed to open test.in")
        .read_to_string(&mut input)
        .expect("Failed to read to string");

    let output = input
        .lines()
        .map(|line| {
            let (id, card) = line.split_once(": ").unwrap();
            let (winning, your) = card.split_once(" | ").unwrap();
            
            let winning_split: Vec<&str> = winning
                .split(" ")
                .filter_map(|number| {
                    if number != "" {
                        Some(number.trim())
                    } else {
                        None
                    }
                })
                .collect();

            let count = your
                .split(" ")
                .filter_map(|number| {
                    if number != "" {
                        Some(number.trim())
                    } else {
                        None
                    }
                })
                .map(|number| {
                    if winning_split.contains(&number) {
                        println!("{} {}", id, number);
                        1
                    } else {
                        0
                    }
                })
                .sum::<usize>();

            if count > 0 {
                println!("{} {}", id, count);
                pow(2, count - 1)
            } else {
                0
            }
        })
        .sum::<u32>();
    
    println!("{}", output);

    Ok(())
}
