use std::{io::{Error, Read}, fs::File};

use queues::{Queue, IsQueue};

#[derive(Copy, Clone)]
struct SpanMap {
    from_start: u64,
    from_end: u64,
    to_start: u64,
}

#[derive(Copy, Clone)]
struct Span {
    start: u64,
    end: u64,
    stage: usize
}


fn main() -> Result<(), Error> {
    let mut input = String::new();
    File::open("test.in")
        .expect("Failed to open test.in")
        .read_to_string(&mut input)
        .expect("Failed to read to string");
    
    let mut mapping: [Vec::<SpanMap>; 7] = Default::default();
    
    let sections: Vec<&str> = input
        .split("\n\n")
        .map(|section| {section.trim()})
        .collect();
    
    let mut i = 1;
    let (_, numx) = sections[0].split_once(": ").unwrap();
    let numx: Vec<u64> = numx.split(" ").map(|x| {x.parse().unwrap()}).collect();
    let mut seeds: Queue<Span> = Queue::new();
    while i < numx.len() {
        seeds.add(Span { start: numx[i - 1], end: numx[i - 1] + numx[i] - 1, stage: 0 });
        i += 2;
    }
    i = 1;

    while i < 8 {
        let (_, nums) = sections[i].split_once(":\n").unwrap();
        mapping[i - 1] = nums
            .lines()
            .map(|line| {
                let nums: Vec<u64> = line
                    .split(" ")
                    .map(|x| {x.parse().unwrap()})
                    .collect();

                SpanMap { from_start: nums[1], from_end: nums[1] + nums[2] - 1, to_start: nums[0] }
            })
        .collect();

        i += 1;
    }

    
    let mut output: u64 = 1000000000000000000;

    i = 1;
    while seeds.size() > 0 {
        let mut seed = seeds.remove().unwrap();
        println!("{} {} {}", seed.start, seed.end, seed.stage);

        if seed.stage == 7 {
            output = output.min(seed.start);
        }
        else {
            let mut cut = false;
            for (_, span) in mapping[seed.stage].iter().enumerate() {
                if seed.start <= span.from_end && seed.end >= span.from_start {
                    cut = true;
                    if span.from_start > seed.start {
                        seeds.add(Span { start: seed.start, end: span.from_start - 1, stage: seed.stage});
                    }
                    if span.from_end < seed.end {
                        seeds.add(Span { start: span.from_end + 1, end: seed.end, stage: seed.stage});
                    }
                    
                    seeds.add(Span { start: span.from_start.max(seed.start) + span.to_start - span.from_start, end: span.from_end.min(seed.end) + span.to_start - span.from_start, stage: seed.stage + 1 });
                    break;
                }
            }
            if cut == false {
                seed.stage += 1;
                seeds.add(seed);
            }
        }

        i += 2;
    }

    println!("{}", output);

    Ok(())
}
