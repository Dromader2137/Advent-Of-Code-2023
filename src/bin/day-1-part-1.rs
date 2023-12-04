use std::{io::{Error, Read}, fs::File};

fn process_line(line: &str) -> u32 {
    let mut line_iter = line.chars().filter_map(|c| {
        c.to_digit(10)
    });
    
    let first = line_iter.next().expect("Should be a number!");
    let second = line_iter.last();

    match second {
        Some(num) => first * 10 + num,
        None => first * 11
    }
}

fn main() -> Result<(), Error> {
    let mut input: String = String::new();
    File::open("test.in")
        .expect("Failed to open input file!")
        .read_to_string(&mut input)
        .expect("Failed to read input file to string!");

    let output = input
        .lines()
        .map(|line| {
            process_line(&line)
        })
        .sum::<u32>();
    
    println!("{}", output.to_string());
    Ok(())
}
