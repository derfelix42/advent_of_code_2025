use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn solve_puzzle_1(lines: Lines<BufReader<File>>) {
    let mut res = 0;
    for line in lines.map_while(Result::ok) {
        for range in line.split(",") {
            let mut start_stop = range.split("-");
            let start = start_stop.next().unwrap().parse::<u64>().unwrap();
            let stop = start_stop.next().unwrap().parse::<u64>().unwrap();
            // println!("-> {} - {}", start, stop);

            for x in start..stop + 1 {
                let string = format!("{x}");
                let len = string.len();
                let half_size = len / 2;

                let first_half = &string[..half_size];
                let last_half = &string[half_size..];
                let same = first_half == last_half;

                if same {
                    // println!("    -> {x}");
                    res += x as u64;
                }
            }
        }
    }
    println!("Puzzle 1 - Result: {}", res)
}

fn solve_puzzle_2(lines: Lines<BufReader<File>>) {
    let mut res = 0;
    for line in lines.map_while(Result::ok) {
        for range in line.split(",") {
            let mut start_stop = range.split("-");
            let start = start_stop.next().unwrap().parse::<u64>().unwrap();
            let stop = start_stop.next().unwrap().parse::<u64>().unwrap();
            // println!("-> {} - {}", start, stop);

            for x in start..stop + 1 {
                let string = format!("{x}");

                for l in 1..string.len() {
                    let subs = string
                        .as_bytes()
                        .chunks(l)
                        .map(|buf| unsafe { str::from_utf8_unchecked(buf).parse::<u64>().unwrap() })
                        .collect::<Vec<u64>>();

                    let all_equal = subs.iter().min() == subs.iter().max();

                    if all_equal {
                        // println!("    -> {:?}", subs);
                        res += x;
                        break;
                    }
                }
            }
        }
    }
    println!("Puzzle 2 - Result: {}", res)
}

fn main() {
    println!("# Advent of Code 2025 | Day 02");

    if let Ok(lines) = read_lines("./input.txt") {
        // solve_puzzle_1(lines);
        solve_puzzle_2(lines);
    }
}
