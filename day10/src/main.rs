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

#[derive(Debug, Clone)]
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

    fn apply_button(&mut self, btn: &Button) {
        for pos in &btn.toggle {
            self.lights[*pos as usize] = !self.lights[*pos as usize];
        }
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

#[derive(Debug, Clone)]
struct Joltage {
    joltages: Vec<u16>,
    goal: Vec<u16>,
}

impl Joltage {
    fn new(goal: &str) -> Self {
        println!("Parsing joltage reading: {goal}");
        let mut goal = goal;
        if goal.starts_with("{") {
            goal = &goal[1..goal.len() - 1];
        }

        let mut goal_vec = Vec::new();
        for c in goal.split(",") {
            let digit = c.parse().unwrap();
            goal_vec.push(digit);
        }

        Self {
            joltages: vec![0; goal_vec.len()],
            goal: goal_vec,
        }
    }

    fn apply_button(&mut self, btn: &Button) {
        for b in &btn.toggle {
            self.joltages[*b as usize] += 1;
        }
    }

    fn apply_buttons_and_check(&self, btns: &Vec<&Button>) -> (bool, bool) {
        let mut joltages = self.joltages.clone();
        for btn in btns {
            for b in &btn.toggle {
                joltages[*b as usize] += 1;
            }
        }

        let is_goal = joltages == self.goal;
        let mut is_too_high = false;
        for i in 0..joltages.len() {
            if joltages[i] > self.goal[i] {
                is_too_high = true;
            }
        }

        (is_goal, is_too_high)
    }

    fn is_goal(&self) -> bool {
        self.joltages == self.goal
    }

    fn is_too_high(&self) -> bool {
        for i in 0..self.joltages.len() {
            if self.joltages[i] > self.goal[i] {
                return true;
            }
        }
        false
    }
}

impl fmt::Display for Joltage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{:?} / {:?}}}", self.joltages, self.goal)
    }
}

fn parse_input(lines: Lines<BufReader<File>>) -> Vec<(Lights, Vec<Button>, Joltage)> {
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

fn solve_puzzle_1(inputs: Vec<(Lights, Vec<Button>, Joltage)>) {
    let mut res: u64 = 0;

    for (lights, buttons, joltages) in &inputs {
        print!("{lights} ");
        for button in buttons {
            print!("{button} ");
        }
        println!("{joltages} ");

        println!("Testing buttons");

        // light, button, cnt
        let mut queue: VecDeque<(Lights, &Button, u64)> = VecDeque::new();
        for button in buttons {
            queue.push_back((lights.clone(), button, 0))
        }

        loop {
            if queue.is_empty() {
                break;
            }
            let first = queue.pop_front();
            if let Some((lights, btn, cnt)) = first {
                let mut lights = lights;
                lights.apply_button(btn);
                let cnt = cnt + 1;
                if lights.is_goal() {
                    println!("Found minimal button presses - {cnt}");
                    res += cnt;
                    break;
                } else {
                    for button in buttons {
                        queue.push_back((lights.clone(), button, cnt))
                    }
                }
            }
        }
    }

    println!("Puzzle 1 - Result: {}", res)
}

fn solve_puzzle_2(inputs: Vec<(Lights, Vec<Button>, Joltage)>) {
    let mut res = 0;

    for (lights, buttons, joltages) in &inputs {
        print!("{lights} ");
        for button in buttons {
            print!("{button} ");
        }
        println!("{joltages} ");

        println!("Testing buttons");

        // light, button, cnt
        let mut queue: VecDeque<(&Joltage, Vec<&Button>, u64)> = VecDeque::new();
        for button in buttons {
            queue.push_back((joltages, vec![button], 0))
        }

        println!("-> Queue length to start with: {}", queue.len());

        // let mut current_depth = 0;
        let mut iterations = 0;
        loop {
            iterations += 1;
            if iterations % 100000 == 0 {
                println!(
                    "-> Queue length after {iterations} iterations: {}",
                    queue.len()
                );
            }
            if queue.is_empty() {
                break;
            }
            let first = queue.pop_front();
            if let Some((orig_joltages, btns, cnt)) = first {
                let mut joltages = orig_joltages.clone();
                let (is_goal, is_too_high) = joltages.apply_buttons_and_check(&btns);
                let cnt = cnt + 1;
                // if current_depth < cnt {
                //     println!("Gooing deeper! {cnt}");
                //     current_depth = cnt;
                // }
                if is_goal {
                    println!(
                        "Found minimal button presses - {cnt} (after {iterations} iterations)"
                    );
                    res += cnt;
                    break;
                } else if is_too_high {
                    // println!("Went {cnt} deep and it was wrong!");
                    continue;
                } else {
                    for button in buttons {
                        let mut btns = btns.clone();
                        btns.push(button);
                        queue.push_back((orig_joltages, btns, cnt))
                    }
                }
            }
        }
    }

    println!("Puzzle 2 - Result: {}", res)
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    println!("# Advent of Code 2025 | Day 10");

    if let Ok(lines) = read_lines("./input.txt") {
        let input: Vec<(Lights, Vec<Button>, Joltage)> = parse_input(lines);
        // solve_puzzle_1(input);
        solve_puzzle_2(input);
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
