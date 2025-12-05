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
    let mut array: Vec<u64> = Vec::new();
    let mut still_in_ranges = true;
    for line in lines.map_while(Result::ok) {
        if line == "" {
            still_in_ranges = false;
            array.sort();
            array.dedup();
            println!("{array:?}");
            continue;
        }

        if still_in_ranges {
            // process new range
            let mut start_stop = line.split("-");
            let start = start_stop.next().unwrap().parse::<u64>().unwrap();
            let stop = start_stop.next().unwrap().parse::<u64>().unwrap();
            println!("-> {} - {}", start, stop);
            for i in start..stop + 1 {
                array.push(i);
            }
        } else {
            // check new id
            let id = line.parse::<u64>().unwrap();
            println!("-> {id}");
            if array.contains(&id) {
                res += 1;
            }
        }
    }
    // println!("{array:?}");

    println!("Puzzle 1 - Result: {}", res)
}

fn solve_puzzle_2(lines: Lines<BufReader<File>>) {
    let mut res = 0;
    // let mut array: Vec<Vec<u8>> = Vec::new();
    // for line in lines.map_while(Result::ok) {
    //     let subs = line
    //         .as_bytes()
    //         .chunks(1)
    //         .map(|buf| unsafe { (str::from_utf8_unchecked(buf) == "@") as u8 })
    //         .collect::<Vec<u8>>();

    //     array.push(subs);
    // }
    // // println!("{array:?}");

    // let height = array.len();
    // let width = array[0].len();

    // let mut iteration_counter = 0;

    // loop {
    //     let last_res = res;
    //     let mut new_array: Vec<Vec<u8>> = Vec::new();
    //     // println!("\n Iteration: {iteration_counter}");
    //     for h in 0..height {
    //         let mut new_line: Vec<u8> = Vec::new();
    //         for w in 0..width {
    //             let val = kernel_sum(&array, h, w, height, width);
    //             let mut new_spot = 0;
    //             if array[h as usize][w as usize] == 1 {
    //                 // print!("{val} ");
    //                 if val < 4 {
    //                     res += 1;
    //                 } else {
    //                     new_spot = 1;
    //                 }
    //             } else {
    //                 // print!(". ");
    //             }
    //             new_line.push(new_spot);
    //         }
    //         new_array.push(new_line);
    //         // println!();
    //     }
    //     array = new_array;
    //     iteration_counter += 1;

    //     if iteration_counter > 100 || last_res == res {
    //         break;
    //     }
    // }

    println!("Puzzle 2 - Result: {res}",)
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    println!("# Advent of Code 2025 | Day 04");

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
