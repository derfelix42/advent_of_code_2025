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

#[derive(Debug)]
struct Lights {
    lights: Vec<bool>,
    goal: Vec<bool>,
}

impl Lights {
    /*
       Todos:
           - Add wanted state?
               -> is_final() -> bool would be possible
    */
    fn init(goal: &str) -> Self {
        let mut goal = goal;
        if goal.starts_with("[") {
            goal = &goal[1..goal.len() - 1];
        }

        let mut goal_vec = Vec::new();
        let chars: Vec<char> = goal.chars().collect();
        for i in 0..goal.len() {
            goal_vec.push(chars[i] == '#');
        }

        Self {
            lights: vec![false; goal_vec.len()],
            goal: goal_vec,
        }
    }

    fn apply_button(&self, btn: Button) {
        todo!()
    }

    fn is_goal(&self) -> bool {
        self.goal == self.lights
    }
}

impl fmt::Display for Lights {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        let mut s = "".to_string();
        for l in &self.lights {
            s += if *l { "#" } else { "." }
        }
        s += "/";
        for l in &self.goal {
            s += if *l { "#" } else { "." }
        }
        write!(f, "[{}]", s)
    }
}

#[derive(Debug)]
struct Button {
    toggle: Vec<u8>,
}

impl Button {
    fn new() -> Self {
        Button { toggle: Vec::new() }
    }
}

impl fmt::Display for Button {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?})", self.toggle)
    }
}

#[derive(Debug)]
struct JoltageReading {
    joltages: Vec<u8>,
}

impl JoltageReading {
    fn new() -> Self {
        Self { joltages: vec![0] }
    }
}

impl fmt::Display for JoltageReading {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{:?}}}", self.joltages)
    }
}

fn parse_input(lines: Lines<BufReader<File>>) -> Vec<(Lights, Vec<Button>, JoltageReading)> {
    let mut input = Vec::new();

    println!("Parsing input");

    for line in lines.map_while(Result::ok) {
        // println!("-> {line}");
        let parts: Vec<&str> = line.split(" ").collect();
        if parts.len() >= 3 {
            let light = Lights::init(parts[0]);
            // println!("  -> parsed lights: {light}");

            let joltages = JoltageReading::new();
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

fn solve_puzzle_1(inputs: Vec<(Lights, Vec<Button>, JoltageReading)>) {
    let mut res = 0;

    for (lights, buttons, joltages) in inputs {
        print!("{lights} ");
        for button in buttons {
            print!("{button} ");
        }
        println!("{joltages} ");
    }

    println!("Puzzle 1 - Result: {}", res)
}

fn solve_puzzle_2(lines: Lines<BufReader<File>>) {
    let mut res = 0;

    println!("Puzzle 2 - Result: {}", res)
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    println!("# Advent of Code 2025 | Day 10");

    if let Ok(lines) = read_lines("./test.txt") {
        let input: Vec<(Lights, Vec<Button>, JoltageReading)> = parse_input(lines);
        solve_puzzle_1(input);
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
