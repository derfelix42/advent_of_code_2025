use std::collections::VecDeque;
use std::fmt;
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

struct Node {
    name: str,
}

fn parse_input(lines: Lines<BufReader<File>>) -> Vec {
    let mut input = Vec::new();

    println!("Parsing input");

    for line in lines.map_while(Result::ok) {
        // println!("-> {line}");
        let parts: Vec<&str> = line.split(" ").collect();
        if parts.len() >= 3 {
            let light = Lights::init(parts[0]);
            // println!("  -> parsed lights: {light}");

            let joltages = Joltage::new(parts[parts.len() - 1]);
            // println!("  -> parsed joltages: {joltages}");

            let mut buttons = Vec::new();
            for i in 1..parts.len() - 1 {
                let mut button = Button::new();
                let btns = parts[i];
                let btns = &btns[1..btns.len() - 1];
                for btn in btns.split(",") {
                    // println!("    -> parsing btn {btn}");
                    button.toggle.push(btn.parse().unwrap());
                }
                buttons.push(button);
            }
            // println!("  -> parsed buttons: {buttons:?}");

            input.push((light, buttons, joltages));
        }
    }

    input
}

fn solve_puzzle_1(inputs: &Vec) {
    let mut res: u64 = 0;

    println!("Puzzle 1 - Result: {}", res)
}

fn solve_puzzle_2(inputs: &Vec) {
    let mut res = 0;

    println!("Puzzle 2 - Result: {}", res)
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    println!("# Advent of Code 2025 | Day 11");

    if let Ok(lines) = read_lines("./test.txt") {
        let input: Vec = parse_input(lines);
        solve_puzzle_1(&input);
        // solve_puzzle_2(input);
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
