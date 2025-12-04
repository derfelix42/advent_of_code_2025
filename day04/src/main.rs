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

fn kernel_sum(array: &Vec<Vec<u8>>, x: usize, y: usize, width: usize, height: usize) -> u8 {
    let mut sum = 0;

    let x = x as i16;
    let y = y as i16;
    let height = height as i16;
    let width = width as i16;

    for i in 0..3_i16 {
        for j in 0..3_i16 {
            let xi = x + i - 1;
            let yj = y + j - 1;
            if xi == x && yj == y {
                // print!("X ");
                continue;
            }
            if xi < 0 || xi >= height {
                // print!(". ");
                continue;
            }
            if yj < 0 || yj >= width {
                // print!(". ");
                continue;
            }

            sum += array[xi as usize][yj as usize];
            // print!("{} ", array[xi as usize][yj as usize]);
        }
        // println!();
    }
    sum
}

fn solve_puzzle_1(lines: Lines<BufReader<File>>) {
    let mut res = 0;
    let mut array: Vec<Vec<u8>> = Vec::new();
    for line in lines.map_while(Result::ok) {
        let subs = line
            .as_bytes()
            .chunks(1)
            .map(|buf| unsafe { (str::from_utf8_unchecked(buf) == "@") as u8 })
            .collect::<Vec<u8>>();

        array.push(subs);
    }
    println!("{array:?}");

    let height = array.len();
    let width = array[0].len();

    for h in 0..height {
        for w in 0..width {
            let val = kernel_sum(&array, h, w, width, height);
            if array[h as usize][w as usize] == 1 {
                print!("{val} ");
                if val < 4 {
                    res += 1;
                }
            } else {
                print!(". ");
            }
        }
        println!();
    }

    println!("Puzzle 1 - Result: {}", res)
}

fn solve_puzzle_2(lines: Lines<BufReader<File>>) {
    let mut res = 0;
    let mut array: Vec<Vec<u8>> = Vec::new();
    for line in lines.map_while(Result::ok) {
        let subs = line
            .as_bytes()
            .chunks(1)
            .map(|buf| unsafe { (str::from_utf8_unchecked(buf) == "@") as u8 })
            .collect::<Vec<u8>>();

        array.push(subs);
    }
    // println!("{array:?}");

    let height = array.len();
    let width = array[0].len();

    let mut iteration_counter = 0;

    loop {
        let last_res = res;
        let mut new_array: Vec<Vec<u8>> = Vec::new();
        // println!("\n Iteration: {iteration_counter}");
        for h in 0..height {
            let mut new_line: Vec<u8> = Vec::new();
            for w in 0..width {
                let val = kernel_sum(&array, h, w, height, width);
                let mut new_spot = 0;
                if array[h as usize][w as usize] == 1 {
                    // print!("{val} ");
                    if val < 4 {
                        res += 1;
                    } else {
                        new_spot = 1;
                    }
                } else {
                    // print!(". ");
                }
                new_line.push(new_spot);
            }
            new_array.push(new_line);
            // println!();
        }
        array = new_array;
        iteration_counter += 1;

        if iteration_counter > 100 || last_res == res {
            break;
        }
    }

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
    fn test_kernel_zeros() {
        let array = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        let h = 1;
        let w = 1;

        let val = kernel_sum(&array, h, w, 3, 3);
        assert_eq!(val, 0, "This should have been 0");
    }

    #[test]
    fn test_kernel_ones() {
        let array = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];
        let h = 1;
        let w = 1;

        let val = kernel_sum(&array, h, w, 3, 3);
        assert_eq!(val, 8, "This should have been 8");
    }

    #[test]
    fn test_kernel_random() {
        let array = vec![vec![1, 1, 1], vec![0, 0, 0], vec![1, 0, 1]];
        let h = 1;
        let w = 1;

        let val = kernel_sum(&array, h, w, 3, 3);
        assert_eq!(val, 5, "This should have been 0");
    }
}
