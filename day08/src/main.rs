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
struct Point {
    x: u32,
    y: u32,
    z: u32,
}

fn point_distance(p1: &Point, p2: &Point) -> f64 {
    let x = (p1.x as f64 - p2.x as f64).powf(2_f64);
    let y = (p1.y as f64 - p2.y as f64).powf(2_f64);
    let z = (p1.z as f64 - p2.z as f64).powf(2_f64);
    let sum = x + y + z;
    sum.sqrt()
}

fn solve_puzzle_1(lines: Lines<BufReader<File>>) {
    let mut res = 1;

    let mut points: Vec<Point> = Vec::new();
    for line in lines.map_while(Result::ok) {
        let parts: Vec<&str> = line.split(",").collect();
        if parts.len() == 3 {
            let x = parts[0].parse::<u32>().unwrap();
            let y = parts[1].parse::<u32>().unwrap();
            let z = parts[2].parse::<u32>().unwrap();
            points.push(Point { x, y, z });
        }
    }

    // make points immutable to preserve order
    let points = points;
    let n = points.len();

    let mut matrix: Vec<Vec<f64>> = vec![vec![0_f64; n]; n];
    let mut edges: Vec<(usize, usize, f64)> = Vec::new();
    for i in 0..n {
        for j in 0..n {
            if i < j || i == j {
                continue;
            }
            let p1 = &points[i];
            let p2 = &points[j];

            let distance = point_distance(p1, p2);

            matrix[i][j] = distance;
            edges.push((i, j, distance))
        }
    }

    edges.sort_by(|a, b| a.2.total_cmp(&b.2));
    let edges = edges;

    // let shortest_edges = &edges[..10];
    println!("Edges: {}", edges.len());

    let mut mst: Vec<Vec<usize>> = Vec::new();
    for edge in &edges[..1000] {
        println!("\n-> {edge:?}");
        let p1 = edge.0;
        let p2 = edge.1;

        let p1_mst = mst.iter().position(|x| x.contains(&p1));
        let p2_mst = mst.iter().position(|x| x.contains(&p2));

        if p1_mst.is_none() && p2_mst.is_none() {
            mst.push(vec![p1, p2]);
        } else if p1_mst.is_some() && p2_mst.is_none() {
            let pos = p1_mst.unwrap();
            mst[pos].push(p2);
        } else if p1_mst.is_none() && p2_mst.is_some() {
            let pos = p2_mst.unwrap();
            mst[pos].push(p1);
        } else {
            // points are in different msts
            let pos1 = p1_mst.unwrap();
            let pos2 = p2_mst.unwrap();

            if pos1 == pos2 {
                continue;
            }

            // append 2 on 1
            let mut snd = mst[pos2].clone(); //.remove(pos2);
            mst[pos1].append(&mut snd);

            mst.remove(pos2);
        }
        println!("    =>  {mst:?}");
    }

    // println!("{points:?}");
    // println!("{matrix:?}");
    // println!("{edges:#?}");

    println!("{mst:#?}");

    // for (i, j, distance) in &edges {
    //     println!("-> ({:?}) - ({:?}) = {distance:>2}", points[*i], points[*j]);
    // }
    println!("{}", edges.len());

    mst.sort_by(|a, b| b.len().cmp(&a.len()));
    for x in &mst[..3] {
        println!("{}", x.len());
        res *= x.len();
    }

    println!("Puzzle 1 - Result: {}", res)
}

fn solve_puzzle_2(lines: Lines<BufReader<File>>) {
    let mut res = 1;

    let mut points: Vec<Point> = Vec::new();
    for line in lines.map_while(Result::ok) {
        let parts: Vec<&str> = line.split(",").collect();
        if parts.len() == 3 {
            let x = parts[0].parse::<u32>().unwrap();
            let y = parts[1].parse::<u32>().unwrap();
            let z = parts[2].parse::<u32>().unwrap();
            points.push(Point { x, y, z });
        }
    }

    // make points immutable to preserve order
    let points = points;
    let n = points.len();

    let mut matrix: Vec<Vec<f64>> = vec![vec![0_f64; n]; n];
    let mut edges: Vec<(usize, usize, f64)> = Vec::new();
    for i in 0..n {
        for j in 0..n {
            if i < j || i == j {
                continue;
            }
            let p1 = &points[i];
            let p2 = &points[j];

            let distance = point_distance(p1, p2);

            matrix[i][j] = distance;
            edges.push((i, j, distance))
        }
    }

    edges.sort_by(|a, b| a.2.total_cmp(&b.2));
    let edges = edges;

    // let shortest_edges = &edges[..10];
    println!("Edges: {}", edges.len());

    let mut mst: Vec<Vec<usize>> = Vec::new();
    for edge in &edges {
        println!("\n-> {edge:?}");
        let p1 = edge.0;
        let p2 = edge.1;

        let p1_mst = mst.iter().position(|x| x.contains(&p1));
        let p2_mst = mst.iter().position(|x| x.contains(&p2));

        if p1_mst.is_none() && p2_mst.is_none() {
            mst.push(vec![p1, p2]);
        } else if p1_mst.is_some() && p2_mst.is_none() {
            let pos = p1_mst.unwrap();
            mst[pos].push(p2);
        } else if p1_mst.is_none() && p2_mst.is_some() {
            let pos = p2_mst.unwrap();
            mst[pos].push(p1);
        } else {
            // points are in different msts
            let pos1 = p1_mst.unwrap();
            let pos2 = p2_mst.unwrap();

            if pos1 == pos2 {
                continue;
            }

            // append 2 on 1
            let mut snd = mst[pos2].clone(); //.remove(pos2);
            mst[pos1].append(&mut snd);

            mst.remove(pos2);
        }
        println!("    =>  {mst:?}");

        if mst.len() == 1 && mst[0].len() == points.len() {
            // Reached the end for puzzle 2 (heuristically)
            let p1 = &points[p1];
            let p2 = &points[p2];

            let mult = p1.x * p2.x;
            println!(
                "        -> Reached the end for puzzle 2 (heuristically): ({p1:?}) ({p2:?}) - {mult}"
            );
            break;
        }
    }

    // println!("{points:?}");
    // println!("{matrix:?}");
    // println!("{edges:#?}");

    // println!("{mst:#?}");

    // for (i, j, distance) in &edges {
    //     println!("-> ({:?}) - ({:?}) = {distance:>2}", points[*i], points[*j]);
    // }
    println!("{}", edges.len());

    // mst.sort_by(|a, b| b.len().cmp(&a.len()));
    // for x in &mst[..3] {
    //     println!("{}", x.len());
    //     res *= x.len();
    // }

    println!("Puzzle 2 - Result: {}", res)
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
    fn test() {}
}
