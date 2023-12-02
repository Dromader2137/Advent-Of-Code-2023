use std::{path::Path, fs::{self, File, read_to_string, OpenOptions}, io::{prelude::*, Read, Error}};

fn main() -> Result<(), Error> {
    let input_path = Path::new("test.in");
    let input = read_to_string(input_path).unwrap().to_string();
    let mut score: i32 = 0;
    for line in input.lines() {
        let mut first_digit: i32 = -1; let mut last_digit: i32 = -1;
        let mut prev = vec![];
        for x in line.chars() {
            prev.push(x);
            let number = x;

            if(prev.len() >= 3) {
                if prev. == "one" {
                    number = '1';
                }
            }

            if number >= '0' && number <= '9' {
                if first_digit == -1 {
                    first_digit = x.to_ascii_lowercase() as i32;
                }
                last_digit = x.to_ascii_lowercase() as i32;
            }
        }
        if first_digit != -1 {
            score += (first_digit - 48) * 10 + last_digit - 48;
        }
    }
    let output_path = Path::new("test.out");
    let output_file = OpenOptions::new().write(true).open(output_path);
    output_file.unwrap().write(score.to_string().as_bytes())?;
    Ok(())
}
