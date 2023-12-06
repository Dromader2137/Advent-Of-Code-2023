use std::{io::{Error, Read}, fs::File};

use num::Float;

fn main() -> Result<(), Error> {
    let mut input = String::new();
    File::open("test.in")
        .expect("expected file to open")
        .read_to_string(&mut input)
        .expect("expected to read to string");

    let mut lines = input.lines();

    let t: Vec<f64> = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| {x.parse::<f64>().unwrap()})
        .collect();

    let s: Vec<f64> = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| {x.parse::<f64>().unwrap()})
        .collect();
    
    let mut sc: u64 = 1;
    let mut i = 0;
    while i < s.len() {
        let cs = s[i];
        let ct = t[i];
        let delta: f64 = ct*ct - 4.0*cs;
        let t1 = ((ct + Float::sqrt(delta)) / 2.0) - 0.00000001;
        let t2 = ((ct - Float::sqrt(delta)) / 2.0) + 0.00000001;

        sc *= (t1.floor() - t2.ceil()) as u64 + 1;
        println!("t^2 - {}t + {} => {} {} / {} {}", ct, cs, t1, t2, t1.floor(), t2.ceil());

        i += 1; 
    }

    println!("{}", sc);

    Ok(())
}
