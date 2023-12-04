use std::{io::{Error, Read}, fs::File};

fn main() -> Result<(), Error> {
    let mut input: String = String::new();
    let _ = File::open("test.in")
        .expect("Failed to open input file!")
        .read_to_string(&mut input)
        .expect("Failed to read input file to string!")
        .to_string()
        .trim();

    let output = input
        .lines()
        .map(|line| {
            let line = line.replace("Game ", "");
            let (_, mut line) = line.split_once(":").expect("Failed to split at :");
            line = line.trim();
            let (mut maxg, mut maxr, mut maxb) = (0, 0, 0);
            let bags = line.split(";").map(|line| {line.trim()});
            for bag in bags {
                let cubes = bag.split(",").map(|cube| {cube.trim()});
                for cube in cubes {
                    let (amount, color) = cube.split_once(" ").expect("Failed to split cube");
                    let amount = amount.parse().expect("Failed to parse amount");
                    if color == "blue" && amount > maxb {
                        maxb = amount;
                    } else if color == "red" && amount > maxr {
                        maxr = amount;
                    } else if color == "green" && amount > maxg {
                        maxg = amount;
                    }
                }
            }
            maxg * maxr * maxb
        })
        .sum::<u32>();

    println!("{}", output.to_string());
    Ok(())
}
