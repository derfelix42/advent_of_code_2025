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
    if let Ok(lines) = read_lines("./melina-input.txt") {
        // Consumes the iterator, returns an (Optional) String
        println!("     ->   50 /   50");
        for mut line in lines.map_while(Result::ok) {
            let direction = if line.chars().next().unwrap() == 'R' {
                1
            } else {
                -1
            };

            let distance = line.split_off(1).parse::<i32>().unwrap() * direction;

            let new_pos = pos + distance;
            let spin_overs = ((distance * direction) as f32 / 100_f32).floor() as i32;
            let mut change = 0;

            if new_pos % 100 == 0 {
                // landet genau auf 0
                if spin_overs == 0 {
                    // ohne "überdrehen"
                    change = 1;
                } else {
                    // mit überdrehen, also auf 0 landen + spinovers
                    change = 1 + spin_overs;
                }
            } else if direction == -1 && new_pos < 0 {
                // nach links drehen und über 0 gegangen
                if spin_overs > 0 {
                    change = 1 + spin_overs;
                } else {
                    change = 1;
                }
            } else if direction == 1 && new_pos > 100 {
                // nach rechts drehen und über 0 gegangen
                if spin_overs > 0 {
                    change = spin_overs;
                } else {
                    change = 1;
                }
            } else if pos == 0 && spin_overs > 0 {
                change = spin_overs;
            }

            cnt += change;

            pos = (new_pos + 100) % 100;
            print!("{:>4} -> {:>4} / {:>4}   ", distance, new_pos, pos);

            println!("{cnt:>4} ({})", change);
            // if cnt > 10 {
            //     break;
            // }
        }
    }

    println!("Count: {}", cnt)
}
