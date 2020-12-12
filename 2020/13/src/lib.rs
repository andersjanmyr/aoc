use std::fs::File;
use std::io::{self, prelude::*};

#[derive(Debug)]
struct Move {
    name: char,
    count: i32,
}

pub fn main() {
    let ls = strings();
    let moves = get_moves(&ls);
    let (x, y) = simulate((10, 1), &moves);
    let dist = x.abs() + y.abs();
    println!("{:?}", dist);
}

fn get_moves(lines: &Vec<String>) -> Vec<Move> {
    lines
        .iter()
        .map(|l| {
            let cs: Vec<char> = l.chars().collect();
            let s: String = cs[1..].into_iter().collect();
            Move {
                name: cs[0],
                count: s.parse::<i32>().unwrap(),
            }
        })
        .collect()
}

fn simulate(waypoint: (i32, i32), moves: &Vec<Move>) -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;
    let mut wx = waypoint.0;
    let mut wy = waypoint.1;
    for m in moves {
        match m.name {
            'N' => wy += m.count,
            'S' => wy -= m.count,
            'E' => wx += m.count,
            'W' => wx -= m.count,
            'R' => {
                let p = turn_right(m.count, wx, wy);
                wx = p.0;
                wy = p.1;
            }
            'L' => {
                let p = turn_left(m.count, wx, wy);
                wx = p.0;
                wy = p.1;
            }
            'F' => {
                let dx = m.count * wx;
                let dy = m.count * wy;
                x += dx;
                y += dy;
            }
            _ => panic!(""),
        }
        println!("{:?} {:?} {:?} {:?}", x, y, wx, wy);
    }
    (x, y)
}

fn turn_right(deg: i32, wx: i32, wy: i32) -> (i32, i32) {
    match deg {
        0 => (wx, wy),
        90 => (wy, -wx),
        180 => (-wx, -wy),
        270 => (-wy, wx),
        _ => panic!(""),
    }
}

fn turn_left(deg: i32, wx: i32, wy: i32) -> (i32, i32) {
    match deg {
        0 => (wx, wy),
        90 => (-wy, wx),
        180 => (-wx, -wy),
        270 => (wy, -wx),
        _ => panic!(""),
    }
}

fn groups() -> Vec<Vec<String>> {
    let mut gs = Vec::new();
    let ls = strings();
    let mut group: Vec<String> = Vec::new();
    for l in ls {
        if l == "" {
            gs.push(group);
            group = Vec::new();
        } else {
            group.push(l);
        }
    }
    gs.push(group);
    gs
}

fn numbers() -> Vec<i32> {
    let lines = strings();
    lines.iter().map(|s| s.parse::<i32>().unwrap()).collect()
}

fn num_matrix() -> Vec<Vec<i32>> {
    let lines = matrix();
    lines
        .iter()
        .map(|ss| ss.iter().map(|s| s.parse::<i32>().unwrap()).collect())
        .collect()
}

fn matrix() -> Vec<Vec<String>> {
    let lines = strings();
    lines
        .iter()
        .map(|l| l.split(",").map(|s| s.to_string()).collect())
        .collect()
}

fn strings() -> Vec<String> {
    let lines = read_lines();
    match lines {
        Ok(lines) => lines.map(|l| l.unwrap()).collect(),
        Err(err) => panic!(err),
    }
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_bin_part() {}

    #[test]
    #[ignore]
    fn test_range() {}

    #[test]
    fn test_main() {
        main();
    }
}
