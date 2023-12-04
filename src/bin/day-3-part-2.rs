use std::{io::{Error, Read}, fs::File};

struct Coords {
    x: usize,
    y: usize
}

struct Number {
    coords_begin: Coords,
    coords_end: Coords,
    num: u32
}

fn main() -> Result<(), Error> {
    let mut input = String::new();
    File::open("test.in")
        .expect("Failed to open file")
        .read_to_string(&mut input)
        .expect("Failed to read to string");

    let mut numbers: Vec<Number> = Vec::new();
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let mut state = array2d::Array2D::filled_with(0u8, height + 2, width + 2);

    for (y, line) in input.lines().enumerate() {
        let mut x = 0;
        let mut ignore_x = 0;
        while x < line.len() {
            state[(y + 1, x + 1)] = line.as_bytes()[x];
            if x < ignore_x {
                x += 1;
                continue;
            }

            for rx in (x..line.len()).rev() {
                let short_line = &line[x..=rx];
                match short_line.parse::<u32>() {
                    Ok(val) => {
                        numbers.push(Number { coords_begin: Coords { x: x, y: y }, 
                                              coords_end: Coords { x: rx, y: y }, 
                                              num: val });
                        ignore_x = rx + 1;
                        break;
                    },
                    Err(_) => {}
                }
            }
            x += 1;
        }
    }
    
    let mut result = 0;
    let mut dy = 1;
    while dy <= width {
        let mut dx = 1;
        while dx <= height {
            // println!("{}", state[(dy, dx)]);
            if state[(dy, dx)] != 42 {
                dx += 1;
                continue;
            }
            
            let mut local_numbers: Vec<u32> = Vec::new();
            for num in numbers.iter() {
                if (num.coords_begin.y <= dy && num.coords_begin.y + 2 >= dy) &&
                   ((num.coords_begin.x <= dx && num.coords_end.x >= dx - 2)){
                    // println!("[{} {}] [{} {}] {}", num.coords_begin.x, num.coords_begin.y, num.coords_end.x, num.coords_end.y, num.num);
                    local_numbers.push(num.num);
                }
            }
            
            if local_numbers.len() == 2 {
                // println!("{} {}", dx, dy);
                // println!("{} {} {}", local_numbers[0], local_numbers[1], local_numbers[0] * local_numbers[1]);
                result += local_numbers[0] * local_numbers[1];
            }

            dx += 1;
        }
        dy += 1;
    }

    println!("{}", result);

    Ok(())
}
