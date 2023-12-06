use std::{io::{Error, Read}, fs::File};

use num::Float;

fn main() -> Result<(), Error> {
    let mut input = String::new();
    File::open("test.in")
        .expect("expected file to open")
        .read_to_string(&mut input)
        .expect("expected to read to string");

    let mut lines = input.lines();

    let t: Vec<&str> = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .collect();

    let mut ts = String::new();
    for ti in t {
        ts += ti;
    }  
    let t: f64 = ts.parse().unwrap();

    let s: Vec<&str> = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .collect();
    
    let mut ss = String::new();
    for si in s {
        ss += si;
    }  
    let s: f64 = ss.parse().unwrap();
    
    let mut sc: u64 = 1;
    let cs = s;
    let ct = t;
    let delta: f64 = ct*ct - 4.0*cs;
    let t1 = ((ct + Float::sqrt(delta)) / 2.0) - 0.00000001;
    let t2 = ((ct - Float::sqrt(delta)) / 2.0) + 0.00000001;

    sc *= (t1.floor() - t2.ceil()) as u64 + 1;
    println!("t^2 - {}t + {} => {} {} / {} {}", ct, cs, t1, t2, t1.floor(), t2.ceil());

    println!("{}", sc);

    Ok(())
}
