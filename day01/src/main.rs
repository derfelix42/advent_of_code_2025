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

    if let Ok(lines) = read_lines("./input.txt") {
        // println!("     ->   50 /   50");
        for mut line in lines.map_while(Result::ok) {
            let direction = if line.chars().next().unwrap() == 'R' {
                1
            } else {
                -1
            };

            let distance = line.split_off(1).parse::<i32>().unwrap();

            for _ in 0..distance {
                pos += direction;
                pos = pos % 100;
                if pos == 0 {
                    cnt += 1;
                }
            }
        }
    }

    println!("Puzzle 2 - Count: {}", cnt)
}
