use std::{io::{Error, Read}, fs::File, collections::HashMap};

use num::integer::gcd_lcm;

fn abs_sub(a: i64, b: i64) -> i64 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn all_on_z(curr: &Vec<&str>) -> bool {
    let mut all = true;
    for x in curr.iter() {
        if !x.ends_with("Z") {
            all = false;
        }
    }
    all
}

fn main() -> Result<(), Error> {
    let mut input: String = String::new();
    File::open("test.in")
        .expect("Failed to open input file!")
        .read_to_string(&mut input)
        .expect("Failed to read input file to string!");

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let (moves, list) = input.split_once("\n\n").unwrap();
    let moves = moves.to_string();

    println!("|{}|", moves);
    for (_, l) in list.lines().enumerate() {
        let (mut a, b) = l.split_once("=").unwrap();
        let (mut l, mut r) = b.split_once(",").unwrap();
        a = a.trim();
        l = l.split_once(" (").unwrap().1;
        r = r.trim().split_once(")").unwrap().0;
        map.insert(a, (l, r));
        println!("|{}|{}|{}|", a, l, r);
    }

    let mut curr: Vec<&str> = list
        .lines()
        .filter_map(|line| {
            let (line, _) = line.split_once(" = ").unwrap();
            if line.ends_with("A") {
                Some(line)
            } else {
                None
            }
        })
        .collect();
    let mut counter: usize = 0;
    let mut score = 0;
    
    println!("{}", curr.len());
   
    let mut fs_vec: Vec<(i64, i64)> = Vec::new();

    for x in curr.iter_mut() {
        counter = 0;
        score = 0;
        while !x.ends_with("Z") {
            score += 1;
            println!("{}", x);
            if moves.chars().nth(counter).unwrap() == 'L' {
                *x = map[*x].0;
            } else {
                *x = map[*x].1;
            }
            counter = (counter + 1) % moves.len();
        }
        let f = score;
        score += 1;
        println!("{}", x);
        if moves.chars().nth(counter).unwrap() == 'L' {
            *x = map[*x].0;
        } else {
            *x = map[*x].1;
        }
        counter = (counter + 1) % moves.len();
        while !x.ends_with("Z") {
            println!("{}", x);
            score += 1;
            if moves.chars().nth(counter).unwrap() == 'L' {
                *x = map[*x].0;
            } else {
                *x = map[*x].1;
            }
            counter = (counter + 1) % moves.len();
        }
        let l = score - f;
        println!("{} {}", f, l);
        fs_vec.push((f, l));
    }

    let mut out = fs_vec[0].clone();
    for (i, a) in fs_vec.iter_mut().enumerate() {
        if i == 0 {
            continue;
        }

        let p = out;
        let c = a.clone();
        
        println!("{} {} {} {}", p.0, p.1, c.0, c.1);
        
        let mut d: i64 = p.0;
        let (_, lcm) = gcd_lcm(p.1, c.1);

        println!("{}", d);
        while (d - c.0) % c.1 != 0 {
            d += p.1;
            println!("{}", d);
        }
        
        out = (d, lcm);
    }

    println!("{}", out.1);

    Ok(())
}
