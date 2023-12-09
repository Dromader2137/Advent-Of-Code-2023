use std::{io::{Error, Read}, fs::{File}};

fn all_zero(vec: &Vec<i32>) -> bool {
    let mut score = true;
    for x in vec.iter() {
        if *x != 0 {
            score = false;
        }
    }
    score
}

fn main() -> Result<(), Error> {
    let mut input = String::new();
    File::open("test.in")
        .expect("Failed to open test.in")
        .read_to_string(&mut input)
        .expect("Failed to read to string");

    let output = input
        .lines()
        .map(|line| {
            let numbers: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
            let mut current: Vec<i32> = Vec::new();
            let mut current_prev: Vec<i32> = Vec::new();
            let mut last: Vec<i32> = vec![*numbers.last().unwrap()];
            let mut i = 0;
            while i < numbers.len() - 1{
                current.push(numbers[i + 1] - numbers[i]);
                i += 1;
            }
            println!("{:?}", current);
            last.push(*current.last().unwrap());
            while !all_zero(&current) {
                current_prev = current.clone();
                current.clear();
                i = 0;
                while i < current_prev.len() - 1{
                    current.push(current_prev[i + 1] - current_prev[i]);
                    i += 1;
                }
                println!("{:?}", current);
                last.push(*current.last().unwrap());
            }
            let mut score = 0;
            for x in last.iter() {
                score += *x;
            }
            score            
        })
        .sum::<i32>();

    println!("{}", output);

    Ok(())
}
