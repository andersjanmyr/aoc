use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*};

pub fn main() {
    let ns: Vec<usize> = strings()[0]
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let mut is: HashMap<usize, usize> = ns[0..ns.len() - 1]
        .iter()
        .enumerate()
        .map(|(i, n)| (*n, i + 1))
        .collect();
    let mut to_insert = (*ns.last().unwrap(), ns.len());
    let mut next_to_insert: (usize, usize);
    for i in ns.len() + 1..30000001 {
        // for i in ns.len() + 1..2021 {
        let index = is.get(&to_insert.0);
        match index {
            None => {
                next_to_insert = (0, i);
            }
            Some(index) => {
                let n = i - index - 1;
                next_to_insert = (n, i);
            }
        }
        if i % 100000 == 0 {
            println!("{:?} {:?}", i, to_insert);
        }
        is.insert(to_insert.0, to_insert.1);
        to_insert = next_to_insert;
    }
    println!("{:?}", to_insert);
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
