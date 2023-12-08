use std::{io::{Error, Read}, fs::File, collections::HashMap};

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

    let mut curr: &str = "AAA";
    let mut counter: usize = 0;
    let mut score = 0;
    while curr != "ZZZ" {
        score += 1;
        if moves.chars().nth(counter).unwrap() == 'L' {
            curr = map[curr].0;
        } else {
            curr = map[curr].1;
        }
        counter = (counter + 1) % moves.len();
    }

    println!("{}", score);

    Ok(())
}
