use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*};

pub fn main() {
    let ns: Vec<usize> = strings()[0]
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let mut is: Vec<(usize, usize)> = ns.iter().enumerate().map(|(i, n)| (*n, i + 1)).collect();
    is.reverse();
    // for i in is.len() + 1..30000001 {
    let mut last = is.first().unwrap().0;
    for i in is.len() + 1..2021 {
        let index = is[1..].iter().find(|(n, _)| *n == last);
        match index {
            None => {
                last = 0;
                is.insert(0, (last, i));
            }
            Some((_, index)) => {
                last = i - index - 1;
                is.insert(0, (last, i));
            }
        }
    }
    println!("{:?}", is);
    println!("{:?}", is.last());
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
