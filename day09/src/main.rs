use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Bytes, Lines};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    x: i64,
    y: i64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Box {
    size: i64,
    a: Point,
    b: Point,
}
impl Box {
    fn new(a: Point, b: Point) -> Self {
        use std::cmp::{max, min};

        let new_a = Point {
            x: min(a.x, b.x),
            y: min(a.y, b.y),
        };
        let new_b = Point {
            x: max(a.x, b.x),
            y: max(a.y, b.y),
        };

        let x_len = (new_b.x - new_a.x).abs() + 1;
        let y_len = (new_b.y - new_a.y).abs() + 1;

        let size = x_len * y_len;

        Self {
            size,
            a: new_a,
            b: new_b,
        }
    }

    fn contains(&self, other: &Box) -> bool {
        self.a.x <= other.a.x
            && self.a.y <= other.a.y
            && self.b.x >= other.b.x
            && self.b.y >= other.b.y
    }
    fn contains_point(&self, point: &Point) -> bool {
        self.a.x <= point.x && point.x <= self.b.x && self.a.y <= point.y && point.y <= self.b.y
    }
}

impl fmt::Display for Box {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "[{}, {}] - {}", self.a, self.b, self.size)
    }
}

fn get_tiles(lines: Lines<BufReader<File>>) -> Vec<Point> {
    let mut tiles = Vec::new();
    for line in lines.map_while(Result::ok) {
        let parts: Vec<&str> = line.split(",").collect();
        if parts.len() == 2 {
            let x = parts[0].parse::<i64>().unwrap();
            let y = parts[1].parse::<i64>().unwrap();
            tiles.push(Point { x, y });
        }
    }
    tiles
}

fn solve_puzzle_1(lines: Lines<BufReader<File>>) {
    let mut res = 0;

    let tiles = get_tiles(lines);

    for i in 0..tiles.len() {
        for j in 0..tiles.len() {
            if i <= j {
                continue;
            }

            let a = tiles[i].clone();
            let b = tiles[j].clone();

            let rect = Box::new(a, b);

            if rect.size > res {
                res = rect.size;
            }
            // println!(
            //     "-> {:?} / {:?} => {x_len} / {y_len}, size: {size}",
            //     (a.x, a.y),
            //     (b.x, b.y)
            // );
        }
    }

    // println!("{tiles:?}");

    println!("Puzzle 1 - Result: {}", res)
}

fn solve_puzzle_2(lines: Lines<BufReader<File>>) {
    let mut res = 0;

    let tiles = get_tiles(lines);
    let min_x = tiles.iter().map(|x| x.x).min();
    let min_y = tiles.iter().map(|x| x.y).min();
    let max_x = tiles.iter().map(|x| x.x).max();
    let max_y = tiles.iter().map(|x| x.y).max();

    println!("Building border:");
    let mut border: Vec<Point> = Vec::new();
    border.push(tiles[tiles.len() - 1].clone());
    for tile in &tiles {
        let last_point = border[border.len() - 1].clone();
        if last_point.x == tile.x {
            let range = if last_point.y > tile.y {
                tile.y..last_point.y
            } else {
                last_point.y..tile.y
            };
            for y in range {
                let new_point = Point {
                    x: last_point.x,
                    y: y,
                };
                // println!("  + {new_point}");
                border.push(new_point);
            }
        }
        if last_point.y == tile.y {
            let range = if last_point.x > tile.x {
                tile.x..last_point.x
            } else {
                last_point.x..tile.x
            };
            for x in range {
                let new_point = Point {
                    x: x,
                    y: last_point.y,
                };
                // println!("  + {new_point}");
                border.push(new_point);
            }
        }
        border.push(tile.clone());
        // println!("-> {tile}");
    }

    border = border
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    border.sort();
    println!("edges: {}", border.len());

    // println!("\nBuilding minimal spanning boxes");
    // let mut box_set = Vec::new();
    // let mut cnt = 0;
    // for i in 0..tiles.len() {
    //     for j in 0..tiles.len() {
    //         if i <= j {
    //             continue;
    //         }

    //         let a = tiles[i].clone();
    //         let b = tiles[j].clone();
    //         let c = &Point { x: a.x, y: b.y };
    //         let d = &Point { x: b.x, y: a.y };

    //         // print!("{a} {b} {c} {d}");
    //         let rect = Box::new(a, b);
    //         // println!(" - {rect}");

    //         if border.contains(c) && border.contains(d) {
    //             box_set.push(rect);
    //         }
    //         print!(".");

    //         if cnt % 1000 == 0 {
    //             println!(
    //                 "{:<2}%",
    //                 ((i * tiles.len() + j) as f64 / (tiles.len() * tiles.len()) as f64)
    //             )
    //         }
    //         cnt += 1;
    //     }
    // }

    println!("\nBuilding Boxes:");
    let mut boxes = Vec::new();
    for i in 0..tiles.len() {
        for j in 0..tiles.len() {
            if i <= j {
                continue;
            }

            let a = tiles[i].clone();
            let b = tiles[j].clone();
            let rect = Box::new(a, b);
            boxes.push(rect);

            // print!("{a} {b} {c} {d}");
            // println!(" - {rect}");

            // if border.contains(c) && border.contains(d) {
            //     println!("  -> is on border");
            // } else {
            //     println!("  -> ignoring");
            // }
            print!(".");
        }
    }

    boxes.sort_by(|a, b| b.size.cmp(&a.size));

    // println!("{boxes}");

    for rect in &boxes {
        // println!("\nTesting box: {rect}");
        let a = &rect.a;
        let b = &rect.b;
        let c = &Point { x: a.x, y: b.y };
        let d = &Point { x: b.x, y: a.y };
        // println!("{a} {b} {c} {d}");

        // a -> run right
        let mut in_wall = true;
        for x in a.x..max_x + 1 {
            if border.contains(Point { x: x, y: a.y }) {
                if in_wall {
                    continue;
                }
            }
        }
    }

    println!("Puzzle 2 - Result: {res}");
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    println!("# Advent of Code 2025 | Day 07");

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
    fn test_box_builder() {
        let test_box = Box::new(Point { x: 4, y: 10 }, Point { x: 1, y: 8 });

        let a = Point { x: 1, y: 8 };
        let b = Point { x: 4, y: 10 };
        let size = 4 * 3;
        let expected_box = Box { a, b, size };
        assert_eq!(test_box, expected_box);
    }

    #[test]
    fn test_contains_point() {
        let test_box = Box::new(Point { x: 4, y: 10 }, Point { x: 2, y: 12 });

        assert_eq!(test_box.contains_point(&Point { x: 1, y: 1 }), false);
        assert_eq!(test_box.contains_point(&Point { x: 3, y: 11 }), true);
        // assert_eq!(test_box.contains_point(&Point { x: 1, y: 1 }), false);
        // assert_eq!(test_box.contains_point(&Point { x: 1, y: 1 }), false);
        // assert_eq!(test_box.contains_point(&Point { x: 1, y: 1 }), false);
    }
}
