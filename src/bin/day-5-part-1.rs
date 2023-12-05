use std::{io::{Error, Read}, fs::File};

#[derive(Copy, Clone)]
struct SpanMap {
    from_start: u32,
    to_start: u32,
    length: u32
}

fn main() -> Result<(), Error> {
    let mut input = String::new();
    File::open("test.in")
        .expect("Failed to open test.in")
        .read_to_string(&mut input)
        .expect("Failed to read to string");
    
    let mut mapping: [Vec::<SpanMap>; 7] = Default::default();
    mapping[0].push(SpanMap { from_start: 1, to_start: 1, length: 1 });
    
    let sections: Vec<&str> = input
        .split("\n\n")
        .map(|section| {section.trim()})
        .collect();
   
    let (_, numx) = sections[0].split_once(": ").unwrap();
    let mut seeds: Vec<u32> = numx
        .split(" ")
        .map(|num| {num.parse().unwrap()})
        .collect();

    for num in seeds.iter() {
        println!("{}", num);
    }

    let mut i = 1;
    while i < 8 {
        let (_, nums) = sections[i].split_once(":\n").unwrap();
        mapping[i - 1] = nums
            .lines()
            .map(|line| {
                let nums: Vec<u32> = line
                    .split(" ")
                    .map(|x| {x.parse().unwrap()})
                    .collect();

                SpanMap { from_start: nums[1], to_start: nums[0], length: nums[2] }
            })
        .collect();

        i += 1;
    }

    i = 0;
    while i < 7 {
        for (id, seed) in seeds.iter_mut().enumerate() {
            println!("  {} {}", id, seed);
            let mut changed = false;
            for (_, span) in mapping[i].iter().enumerate() {
                // println!("{} {} {}", span.from_start, span.to_start, span.length);
                if *seed >= span.from_start && *seed < span.from_start + span.length && changed == false {
                    println!("    {}", seed);
                    *seed = span.to_start + (*seed - span.from_start);
                    changed = true;
                    println!("    {}", seed);
                }
            }

        }
        for (id, seed) in seeds.iter_mut().enumerate() {
                println!("{} {}", id, seed);
            }

        i += 1;
    }

    let output = seeds
        .iter()
        .min()
        .unwrap();

    println!("{}", output);

    Ok(())
}
