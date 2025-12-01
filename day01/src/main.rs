use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    println!("# Advent of Code 2025 | Day 01");

    let mut pos = 50;
    let mut cnt = 0;

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        println!("     ->   50 /   50");
        for mut line in lines.map_while(Result::ok) {
            let direction = line.chars().next().unwrap();
            let mut distance = line.split_off(1).parse::<i32>().unwrap();
            if direction == 'L' {
                distance = distance * -1;
            }
            let new_pos = pos + distance;

            let mut zero_change = false;
            let mut zero = false;

            if new_pos == 0 || new_pos == 100 {
                zero = true;
            } else if pos > 0 && new_pos < 0 {
                zero_change = true;
            } else if new_pos > 100 {
                zero_change = true;
            }

            if zero || zero_change {
                cnt += 1;
            }

            pos = (new_pos + 100) % 100;
            print!("{:>4} -> {:>4} / {:>4}   ", distance, new_pos, pos);
            if zero {
                print!("*0*");
            }

            if zero_change {
                print!("change")
            }

            println!();
            if cnt > 10 {
                break;
            }
        }
    }

    println!("Count: {}", cnt)
}
