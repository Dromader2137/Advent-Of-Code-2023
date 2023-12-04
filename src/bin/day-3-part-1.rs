use std::{io::{Error, Read}, fs::File};

fn main() -> Result<(), Error> {
    let mut input: String = String::new();
    File::open("test.in")
        .expect("Failed to open input file!")
        .read_to_string(&mut input)
        .expect("Failed to read input file to string!");
    let input = input.trim();




    let output: u32 = 0;

    println!("{}", output.to_string());
    Ok(())
}
