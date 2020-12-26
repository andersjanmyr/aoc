use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Dir {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

impl Dir {
    fn to_coord(self) -> (i32, i32) {
        match self {
            E => (1, 0),
            SE => (1, 1),
            SW => (0, 1),
            W => (-1, 0),
            NW => (-1, -1),
            NE => (0, -1),
        }
    }

    fn dirs() -> Vec<Dir> {
        vec![E, SE, SW, W, NW, NE]
    }
}

use Dir::*;

fn main() {
    let input = read_to_string("input.txt");
    let data = input.unwrap();
    let v = parse_input(&data);
    let mut tiles = HashSet::new();
    for dirs in v {
        let coord = dirs.iter().fold((0, 0), |acc, &d| {
            let c = d.to_coord();
            (acc.0 + c.0, acc.1 + c.1)
        });
        println!("C: {:?}", coord);
        if !tiles.insert(coord) {
            tiles.remove(&coord);
        }
    }
    println!("Star 1: {}", tiles.len());

    let dirs = Dir::dirs();
    for n in 1..=100 {
        let mut to_white = Vec::new();
        let mut to_black = Vec::new();
        let min = tiles.iter().fold((i32::MAX, i32::MAX), |a, i| {
            let x = if a.0 < i.0 { a.0 } else { i.0 };
            let y = if a.1 < i.1 { a.1 } else { i.1 };
            (x - 1, y - 1)
        });
        let max = tiles.iter().fold((i32::MIN, i32::MIN), |a, i| {
            let x = if a.0 > i.0 { a.0 } else { i.0 };
            let y = if a.1 > i.1 { a.1 } else { i.1 };
            (x + 1, y + 1)
        });
        for i in min.0..max.0 {
            for j in min.1..max.1 {
                let adj = dirs.iter().map(|d| {
                    let (x, y) = d.to_coord();
                    (x + i, y + j)
                });
                let black = tiles.contains(&(i, j));
                let count = adj.filter(|c| tiles.contains(c)).count();
                if black && (count == 0 || count > 2) {
                    to_white.push((i, j));
                }
                if !black && count == 2 {
                    to_black.push((i, j));
                }
            }
        }
        for p in to_white {
            tiles.remove(&p);
        }
        for p in to_black {
            tiles.insert(p);
        }
        println!("Star 2: {} {}", n, tiles.len());
    }
}

fn parse_input(s: &str) -> Vec<Vec<Dir>> {
    s.lines().map(|l| parse_line(l)).collect()
}

fn parse_line(s: &str) -> Vec<Dir> {
    let mut dirs = Vec::new();
    let cs: Vec<_> = s.chars().collect();
    let mut i = 0;
    while i < cs.len() {
        if cs[i] == 'n' || cs[i] == 's' {
            dirs.push(s_to_dir(&cs[i..i + 2]));
            i += 2;
        } else {
            dirs.push(s_to_dir(&cs[i..i + 1]));
            i += 1;
        }
    }
    dirs
}

fn s_to_dir(s: &[char]) -> Dir {
    match s {
        ['e'] => E,
        ['s', 'e'] => SE,
        ['s', 'w'] => SW,
        ['w'] => W,
        ['n', 'w'] => NW,
        ['n', 'e'] => NE,
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
