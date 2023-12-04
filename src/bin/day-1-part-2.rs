use std::{io::{Error, Read}, fs::File};

fn process_line(line: &str) -> u32 {
    let mut line_iter = (0..line.len()).filter_map(|index| {
        let line_slice = &line[index..];
        let result = if line_slice.starts_with("one") {
            '1'
        } else if line_slice.starts_with("two") {
            '2'
        } else if line_slice.starts_with("three") {
            '3'
        } else if line_slice.starts_with("four") {
            '4'
        } else if line_slice.starts_with("five") {
            '5'
        } else if line_slice.starts_with("six") {
            '6'
        } else if line_slice.starts_with("seven") {
            '7'
        } else if line_slice.starts_with("eight") {
            '8'
        } else if line_slice.starts_with("nine") {
            '9'
        } else {
            line_slice.chars().next().unwrap()
        };
        result.to_digit(10)
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
