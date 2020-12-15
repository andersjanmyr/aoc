use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*};

pub struct Memory {
    ns: Vec<usize>,
}

impl Memory {
    fn new(ns: Vec<usize>) -> Self {
        Self { ns }
    }
}

pub struct MemoryIntoIterator {
    memory: Memory,
    index: usize,
    last_spoken: usize,
    map: HashMap<usize, usize>,
}

impl IntoIterator for Memory {
    type Item = usize;
    type IntoIter = MemoryIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        let map: HashMap<usize, usize> = self.ns[0..self.ns.len() - 1]
            .iter()
            .enumerate()
            .map(|(i, n)| (*n, i + 1))
            .collect();
        let last_spoken = *self.ns.last().unwrap();
        MemoryIntoIterator {
            memory: self,
            index: 0,
            map,
            last_spoken,
        }
    }
}

impl Iterator for MemoryIntoIterator {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.index < self.memory.ns.len() {
            let result = self.memory.ns[self.index];
            self.index += 1;
            return Some(result);
        }
        self.last_spoken = self.index
            - self
                .map
                .insert(self.last_spoken, self.index)
                .unwrap_or(self.index);
        self.index += 1;
        // println!("{:?} {:?}", self.turn, self.last_spoken);
        Some(self.last_spoken)
    }
}

pub fn main() {
    let ns: Vec<usize> = strings()[0]
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    let mem = Memory::new(ns);
    let result = mem.into_iter().take(30000000).last();
    println!("{:?}", result);
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

fn numbers() -> Vec<usize> {
    let lines = strings();
    lines.iter().map(|s| s.parse::<usize>().unwrap()).collect()
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
