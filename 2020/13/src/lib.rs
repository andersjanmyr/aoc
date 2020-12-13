use std::fs::File;
use std::io::{self, prelude::*};

fn inv_mod(x: i64, p: i64) -> i64 {
    // p must be prime
    (0..p - 2).fold(1, |o, _| (o * x) % p)
}
pub fn main() {
    let ls = strings();
    let buses: Vec<(i64, i64)> = ls[1]
        .split(',')
        .enumerate()
        .filter_map(|(i, l)| l.parse().ok().map(|b| (i as i64, b)))
        .collect();

    let prod: i64 = buses.iter().map(|(_, b)| b).product();

    let result2 = buses
        .iter()
        .map(|&(a, b)| -a * (prod / b) * inv_mod(prod / b, b))
        .sum::<i64>()
        .rem_euclid(prod);

    println!("{:?}", result2);
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

fn numbers() -> Vec<i64> {
    let lines = strings();
    lines.iter().map(|s| s.parse::<i64>().unwrap()).collect()
}

fn num_matrix() -> Vec<Vec<i64>> {
    let lines = matrix();
    lines
        .iter()
        .map(|ss| ss.iter().map(|s| s.parse::<i64>().unwrap()).collect())
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
