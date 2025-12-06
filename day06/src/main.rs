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
    let mut raw_sheet: Vec<String> = Vec::new();
    let mut sheet: Vec<Vec<String>> = Vec::new();
    for line in lines.map_while(Result::ok) {
        raw_sheet.push(line);
    }

    let operators = raw_sheet.len() - 1;
    let mut i = 0;
    loop {
        let mut current_problem = Vec::new();
        if i >= raw_sheet[0].len() - 1 {
            break;
        }

        let mut next_i = i + 1;
        loop {
            if next_i >= raw_sheet[0].len() {
                break;
            }

            let op = raw_sheet[operators].chars().nth(next_i).unwrap();
            println!("Op: {op}, curr_i = {next_i}, {}", op == '*' || op == '+');
            if op == '*' || op == '+' {
                break;
            }

            next_i += 1;
        }

        println!("Got out of next_i search at {next_i}");

        for j in (i..next_i).rev() {
            print!("{j}: ");
            let mut num_string = String::new();
            for y in 0..operators {
                let st = &raw_sheet[y].chars().nth(j).unwrap().to_string();
                num_string += st;
                print!("{st}");
            }
            println!();

            let x = num_string.trim();
            if x != "" {
                current_problem.push(x.to_owned());
            }
        }

        current_problem.push(raw_sheet[operators].chars().nth(i).unwrap().to_string());
        println!("Current processed Problem is: {:?}\n", current_problem);
        sheet.push(current_problem);

        i = next_i;
    }

    println!("{sheet:?}");

    for problem in sheet {
        let op = &problem[problem.len() - 1];

        if op == "+" {
            print!("Addition ({}):", problem.len());
            let mut sum = 0;
            for i in 0..problem.len() - 1 {
                let num = problem[i].parse::<u64>().unwrap();
                print!("{num} ");
                sum += num;
            }
            println!(" = {sum}");
            res += sum;
        }

        if op == "*" {
            print!("Multiplication ({}):", problem.len());
            let mut sum = 1;
            for i in 0..problem.len() - 1 {
                let num = problem[i].parse::<u64>().unwrap();
                print!("{num} ");
                sum *= num;
            }
            println!(" = {sum}");
            res += sum;
        }
    }
    println!("Puzzle 2 - Result: {res}",)
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    println!("# Advent of Code 2025 | Day 06");

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
