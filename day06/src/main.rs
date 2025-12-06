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
    let mut sheet: Vec<Vec<String>> = Vec::new();
    for line in lines.map_while(Result::ok) {
        let mut output = String::new();
        let mut blank = true;
        output.extend(
            line.split_whitespace()
                .inspect(|_| blank = false)
                .flat_map(|word| [word, " "]),
        );

        let test = output
            .split(' ')
            .into_iter()
            .map(|a| a.to_string())
            .filter(|x| x != "")
            .collect();
        sheet.push(test);

        // println!("line: {output}");

        // if line == "" {
        //     still_in_ranges = false;
        //     // array.sort();
        //     // array.dedup();
        //     println!("{ranges:?}");
        //     continue;
        // }

        // if still_in_ranges {
        //     // process new range
        //     let mut start_stop = line.split("-");
        //     let start = start_stop.next().unwrap().parse::<u64>().unwrap();
        //     let stop = start_stop.next().unwrap().parse::<u64>().unwrap();
        //     ranges.push((start, stop));
        // } else {
        //     // check new id
        //     let id = line.parse::<u64>().unwrap();
        //     print!("-> {id}");
        //     for (start, stop) in &ranges {
        //         if id >= *start && id <= *stop {
        //             res += 1;
        //             print!(" good");
        //             break;
        //         }
        //     }
        //     println!();
        // }
    }

    println!("{sheet:?}");

    for problem in 0..sheet[0].len() {
        let mut numbers = Vec::new();

        for i in 0..sheet.len() {
            let val = &sheet[i][problem];
            if val == "*" {
                // print!("Multiplication: ");
                let mut sum = 1;
                for x in &numbers {
                    sum *= x;
                    // print!("{x} ");
                }
                // println!("{sum}");
                res += sum;
            } else if val == "+" {
                // print!("Addition: ");
                let mut sum = 0;
                for x in &numbers {
                    sum += x;
                    // print!("{x} ");
                }
                // println!("{sum}");
                res += sum;
            } else {
                let num = val.parse::<u64>().unwrap();
                // println!("-> {num}");
                numbers.push(num);
            }
        }
    }

    println!("Puzzle 1 - Result: {}", res)
}

fn solve_puzzle_2(lines: Lines<BufReader<File>>) {
    let mut res = 0;
    // let mut ranges: Vec<(u64, u64)> = Vec::new();
    // for line in lines.map_while(Result::ok) {
    //     if line == "" {
    //         break;
    //     }

    //     let mut start_stop = line.split("-");
    //     let start = start_stop.next().unwrap().parse::<u64>().unwrap();
    //     let stop = start_stop.next().unwrap().parse::<u64>().unwrap();
    //     ranges.push((start, stop));
    // }
    // ranges.sort_by(|a, b| a.0.cmp(&b.0));

    // // De-interlace ranges
    // let mut ignore_list: Vec<(u64, u64)> = Vec::new();
    // for i in 0..ranges.len() {
    //     let (mut curr_start, mut curr_stop) = *&ranges[i];
    //     println!("\nChecking {curr_start} - {curr_stop}");

    //     for (start, stop) in &ranges[i + 1..] {
    //         println!("  -> {start} - {stop}");
    //         if ignore_list.contains(&(*start, *stop)) {
    //             continue;
    //         }

    //         if start >= &curr_start && start <= &curr_stop && stop > &curr_stop {
    //             // end needs to be extended
    //             curr_stop = *stop;
    //             ranges[i].1 = curr_stop;
    //             ignore_list.push((*start, *stop));
    //             println!(
    //                 "    (END needs to be extended) => {}-{}",
    //                 curr_start, curr_stop
    //             );
    //         }

    //         if stop >= &curr_start && stop <= &curr_stop && start < &curr_start {
    //             // start needs to be extended
    //             curr_start = *start;
    //             ignore_list.push((*start, *stop));
    //             println!(
    //                 "    (START needs to be extended) => {}-{}",
    //                 curr_start, curr_stop
    //             );
    //         }
    //     }
    //     println!("New Range: {} - {}", curr_start, curr_stop)
    // }

    // println!("Final fresh ranges:");
    // for (start, stop) in &ranges {
    //     if ignore_list.contains(&(*start, *stop)) {
    //         continue;
    //     }

    //     println!("-> {start} - {stop} ({})", stop - start + 1);

    //     res += stop - start + 1;
    // }

    // // println!("{ranges:?}");

    println!("Puzzle 2 - Result: {res}",)
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    println!("# Advent of Code 2025 | Day 06");

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
