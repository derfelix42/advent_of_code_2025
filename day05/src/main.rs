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
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut still_in_ranges = true;
    for line in lines.map_while(Result::ok) {
        if line == "" {
            still_in_ranges = false;
            // array.sort();
            // array.dedup();
            println!("{ranges:?}");
            continue;
        }

        if still_in_ranges {
            // process new range
            let mut start_stop = line.split("-");
            let start = start_stop.next().unwrap().parse::<u64>().unwrap();
            let stop = start_stop.next().unwrap().parse::<u64>().unwrap();
            ranges.push((start, stop));
        } else {
            // check new id
            let id = line.parse::<u64>().unwrap();
            print!("-> {id}");
            for (start, stop) in &ranges {
                if id >= *start && id <= *stop {
                    res += 1;
                    print!(" good");
                    break;
                }
            }
            println!();
        }
    }
    // println!("{array:?}");

    println!("Puzzle 1 - Result: {}", res)
}

fn solve_puzzle_2(lines: Lines<BufReader<File>>) {
    let mut res = 0;
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    for line in lines.map_while(Result::ok) {
        if line == "" {
            break;
        }

        let mut start_stop = line.split("-");
        let start = start_stop.next().unwrap().parse::<u64>().unwrap();
        let stop = start_stop.next().unwrap().parse::<u64>().unwrap();
        ranges.push((start, stop));
    }
    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut new_ranges = Vec::new();

    for (start, stop) in ranges {
        println!("\nChecking range {start}-{stop}");
        if new_ranges.len() == 0 {
            new_ranges.push((start, stop));
            println!("-> empty new_ranges - just appending {:?}", new_ranges);
            continue;
        }

        let mut overlapping = Vec::new();

        for i in 0..new_ranges.len() {
            let n_start = &new_ranges[i].0;
            let n_stop = &new_ranges[i].1;

            // same range
            if &start == n_start && &stop == n_stop {
                overlapping.push(i);
                continue;
            }

            if &start >= n_start && &start <= n_stop || &stop >= n_start && &stop <= n_stop {
                overlapping.push(i);
            }
        }

        match overlapping.len() {
            0 => {
                new_ranges.push((start, stop));
                println!(
                    "-> overlapping with nothing, just appending: {:?}",
                    new_ranges
                );
            }
            1 => {
                let n_start = new_ranges[overlapping[0]].0;
                let n_stop = new_ranges[overlapping[0]].1;
                print!("-> overlapping with another: {n_start}-{n_stop}");

                if start >= n_start && start <= n_stop && stop > n_stop {
                    new_ranges[overlapping[0]].1 = stop;
                }

                if stop >= n_start && stop <= n_stop && start < n_start {
                    new_ranges[overlapping[0]].0 = start;
                }
                println!(" => {:?}", new_ranges);
            }
            2 => {
                let n_start_1 = new_ranges[overlapping[0]].0;
                let n_stop_1 = new_ranges[overlapping[0]].1;

                let n_start_2 = new_ranges[overlapping[1]].0;
                let n_stop_2 = new_ranges[overlapping[1]].1;
                println!(
                    "-> Overlapping with two ranges: {}-{} / {}-{}",
                    n_start_1, n_stop_1, n_start_2, n_stop_2
                );

                if n_start_1 < n_start_2 {
                    new_ranges[overlapping[0]].1 = n_stop_2;
                    new_ranges.remove(overlapping[1]);
                } else {
                    new_ranges[overlapping[0]].0 = n_start_2;
                    new_ranges.remove(overlapping[1]);
                }

                println!(
                    "-> merged range: {}-{}",
                    new_ranges[overlapping[0]].0, new_ranges[overlapping[0]].1
                );
                println!("=> {:?}", new_ranges);

                // unimplemented!("Overlapping over two new_ranges!")
            }
            x => {
                panic!("Unexpected number of overlappings! {x}")
            }
        }
    }

    println!("Final fresh ranges:");
    for (start, stop) in &new_ranges {
        println!("-> {start} - {stop} ({})", stop - start + 1);

        res += stop - start + 1;
    }

    // println!("{ranges:?}");

    println!("Puzzle 2 - Result: {res}",)
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    println!("# Advent of Code 2025 | Day 04");

    if let Ok(lines) = read_lines("./input.txt") {
        // solve_puzzle_1(lines);
        solve_puzzle_2(lines);
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
