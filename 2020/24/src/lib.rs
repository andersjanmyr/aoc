use itertools::Itertools;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
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
    println!("Star 1: {}", tiles.len())

    // let mut cups = Cups::new(1_000_000, v.clone());
    // for _ in 0..10_000_000 {
    //     cups.step()
    // }
    // println!("Star 2: {}", cups.next_after(1).take(2).product::<usize>());
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
