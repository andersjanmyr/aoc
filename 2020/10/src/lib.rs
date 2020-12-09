use combinations::Combinations;
use std::fs::File;
use std::io::{self, prelude::*};

pub fn main() {
    let ns = numbers();

    let (v, i) = find(&ns, 25);
    println!("{:?} {:?} {:?}", ns, i, v);
    let (i, j, min, max) = find_first_sum(&ns, v);
    println!("{:?} {:?} {:?} {:?} {:?}", i, j, min, max, min + max);
}

fn find(ns: &Vec<i64>, l: usize) -> (i64, usize) {
    for i in 0..(ns.len() - l) {
        let sums: Vec<i64> = Combinations::new(ns[i..(i + l)].to_vec(), 2)
            .map(|v| v[0] + v[1])
            .collect();
        let n = ns[i + l];
        if !sums.contains(&n) {
            return (n, i);
        }
    }
    panic!("");
}

fn find_first_sum(ns: &Vec<i64>, s: i64) -> (usize, usize, i64, i64) {
    for i in 0..ns.len() {
        for j in i + 1..ns.len() {
            let mut v = ns[i..j].to_vec();
            let sum = v.iter().fold(0, |a, i| a + i);
            if sum == s {
                v.sort();
                return (i, j - 1, v[0], v[v.len() - 1]);
            }
            if sum > s {
                break;
            }
        }
    }
    panic!("");
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
