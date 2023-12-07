use std::{io::{Error, Read}, fs::File};

struct Comb {
    t: u8,
    fb: String,
}

fn parse(coms: String, coms_bck: String, value: u32) -> (Comb, u32) {
    let mut com: Vec<u8> = coms_bck.chars().map(|x| {
        match x {
            'A' => 14u8,
            'K' => 13u8,
            'Q' => 12u8,
            'J' => 11u8,
            'T' => 10u8,
            _ => x.to_digit(10).unwrap() as u8
        }
    }).collect();
    com.sort_by(|a, b| b.cmp(a));

    let jcount: u32 = coms_bck.chars().filter_map(|x| {
        if x == 'J' {
            Some(1)
        } else {
            None
        }
    }).sum();

            
    if com[0] == com[1] && com[1] == com[2] && com[2] == com[3] && com[3] == com[4] {
        (Comb {t: 7, fb: coms.to_string()}, value)
    } else if (com[0] == com[1] && com[1] == com[2] && com[3] == com[4]) ||
              (com[0] == com[1] && com[2] == com[3] && com[3] == com[4]) {
        (Comb {t: 5, fb: coms.to_string()}, value)
    } else if (com[0] == com[1] && com[1] == com[2] && com[2] == com[3]) ||
              (com[1] == com[2] && com[2] == com[3] && com[3] == com[4]) {
        (Comb {t: 6, fb: coms.to_string()}, value)
    } else if (com[0] == com[1] && com[1] == com[2]) ||
              (com[1] == com[2] && com[2] == com[3]) ||
              (com[2] == com[3] && com[3] == com[4]) {
        (Comb {t: 4, fb: coms.to_string()}, value)
    } else if (com[0] == com[1] && com[2] == com[3]) ||
              (com[1] == com[2] && com[3] == com[4]) ||
              (com[0] == com[1] && com[3] == com[4]){
        (Comb {t: 3, fb: coms.to_string()}, value)
    } else if com[0] == com[1] || com[1] == com[2] {
        (Comb {t: 2, fb: coms.to_string()}, value)
    } else if com[2] == com[3] || com[3] == com[4] {
        (Comb {t: 2, fb: coms.to_string()}, value)
    } else {
        (Comb {t: 1, fb: coms.to_string()}, value)
    }
}

fn main() -> Result<(), Error> {
    let mut input = String::new();
    File::open("test.in")
        .expect("expected file to open")
        .read_to_string(&mut input)
        .expect("expected to read to string");

    let mut output: Vec<(Comb, u32)> = input
        .lines()
        .map(|line| {
            let (coms, value) = line.split_once(" ").unwrap();
            println!("|{}| |{}|", coms, value);
            let value: u32 = value.parse().unwrap();
            let mut coms = coms.to_string();
            let coms_bck = coms.clone();
            
            coms = coms.replace("A", "F");
            coms = coms.replace("K", "E");
            coms = coms.replace("Q", "D");
            coms = coms.replace("J", "C");
            coms = coms.replace("T", "B");

            parse(coms, coms_bck, value)
        })
        .collect();


    output.sort_by(|(a, va), (b, vb)| {
        if a.t != b.t {
            a.t.cmp(&b.t)
        } else if a.fb != b.fb {
            a.fb.cmp(&b.fb)
        } else {
            va.cmp(&vb).reverse()
        }
    });

    for (a, b) in output.iter() {
        println!("{} {} {}", a.t, a.fb, b);
    }
    
    let score: u64 = output.iter()
        .enumerate()
        .map(|(i, (_, b))| {
            *b as u64 * ((i + 1) as u64)
        })
        .sum();

    println!("{}", score);

    Ok(())
}
