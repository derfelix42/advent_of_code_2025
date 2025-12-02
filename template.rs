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

fn solve_puzzle_1() {}

fn solve_puzzle_2() {}

fn main() {
    println!("# Advent of Code 2025 | Day 02");

    let res = 0;

    if let Ok(lines) = read_lines("./melina-input.txt") {
        for mut line in lines.map_while(Result::ok) {}
    }

    println!("Result: {}", res)
}
