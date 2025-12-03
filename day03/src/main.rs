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
        let subs = line
            .as_bytes()
            .chunks(1)
            .map(|buf| unsafe { str::from_utf8_unchecked(buf).parse::<u64>().unwrap() })
            .collect::<Vec<u64>>();

        let max = subs[..subs.len() - 1].iter().max().unwrap();
        let first_max_pos = subs.iter().position(|n| n == max).unwrap();
        let sec_max = subs[first_max_pos + 1..].iter().max().unwrap();
        let val = max * 10 + sec_max;
        res += val;
        println!("{subs:?} - {val}");
    }
    println!("Puzzle 1 - Result: {}", res)
}

fn solve_puzzle_2(lines: Lines<BufReader<File>>) {
    let mut res = 0;
    for line in lines.map_while(Result::ok) {
        let subs = line
            .as_bytes()
            .chunks(1)
            .map(|buf| unsafe { str::from_utf8_unchecked(buf).parse::<u8>().unwrap() })
            .collect::<Vec<u8>>();

        let mut val = 0;
        let mut curr_pos = 0;
        for i in 0..12 {
            let offset = 11 - i;
            let current_slice = &subs[curr_pos..subs.len() - offset];
            let max = current_slice.iter().max().unwrap();
            curr_pos += current_slice.iter().position(|n| n == max).unwrap() + 1;
            val += *max as u64 * 10_u64.pow(offset as u32);
            // println!(
            //     "  -> {i:>2} / {offset:>2} | {max}, {curr_pos}, {:?}",
            //     current_slice
            // );
        }
        res += val;
        // println!("{subs:?} - {val}");
    }
    println!("Puzzle 2 - Result: {res}",)
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    println!("# Advent of Code 2025 | Day 03");

    if let Ok(lines) = read_lines("./input.txt") {
        // solve_puzzle_1(lines);
        solve_puzzle_2(lines);
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
