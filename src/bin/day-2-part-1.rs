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
            let (id_str, mut line) = line.split_once(":").expect("Failed to split at :");
            let id: u32 = id_str.parse().expect("Failed to parse id");
            line = line.trim();
            // println!("|{}|", line);
            let result = line.split(";")
                .map(|bag| {
                    // println!("|{}|", bag);
                    bag.split(",")
                        .map(|color| {
                            let color = color.trim();
                            if color.ends_with("blue") && color.split_once(" ").unwrap().0.parse::<u32>().unwrap() > 14 {
                                0
                            } else if color.ends_with("green") && color.split_once(" ").unwrap().0.parse::<u32>().unwrap() > 13 {
                                0
                            } else if color.ends_with("red") && color.split_once(" ").unwrap().0.parse::<u32>().unwrap() > 12 {
                                0
                            } else {
                                id
                            }
                        })
                        .min()
                        .unwrap()
                })
                .min()
                .unwrap();
            result
        })
        .sum::<u32>();

    println!("{}", output.to_string());
    Ok(())
}
