use std::collections::HashSet;
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

    let mut beams: Vec<usize> = Vec::new();
    for line in lines.map_while(Result::ok) {
        println!("{line}");
        if beams.len() == 0 {
            // beams = vec![false; line.len()];
            let start_pos = line.chars().position(|c| c == 'S').unwrap();
            beams.push(start_pos);
            println!("-> {beams:?}");
            continue;
        } else {
            let chars: Vec<char> = line.chars().into_iter().collect();

            let mut new_beams = Vec::new();
            for beam in beams {
                if chars[beam] == '^' {
                    new_beams.push(beam + 1);
                    new_beams.push(beam - 1);
                    res += 1;
                } else {
                    new_beams.push(beam);
                }
            }

            beams = new_beams
                .into_iter()
                .collect::<HashSet<_>>()
                .into_iter()
                .collect();
        }

        println!("-> {beams:?}");
    }

    println!("Puzzle 1 - Result: {}", res)
}

fn solve_puzzle_2(lines: Lines<BufReader<File>>) {
    let mut res = 0;

    println!("Puzzle 2 - Result: {res}",)
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    println!("# Advent of Code 2025 | Day 07");

    if let Ok(lines) = read_lines("./input.txt") {
        solve_puzzle_1(lines);
        // solve_puzzle_2(lines);
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
