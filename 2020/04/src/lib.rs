use std::fs::File;
use std::io::{self, prelude::*};

pub fn main() {
    let v = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    let ts: Vec<usize> = v.iter().map(|(d, r)| trees(*d, *r)).collect();
    let prod = ts.iter().fold(1, |a, b| a * b);
    println!("{:?}, {:?}", ts, prod);
}

fn trees(d: usize, r: usize) -> usize {
    let ls = strings();
    let width = ls[0].len();
    let mut y = 0;
    let mut trees = 0;
    for i in (d..ls.len()).step_by(d) {
        y = (y + r) % width;
        let l = &ls[i];
        let c = l.chars().nth(y).unwrap();
        if c == '#' {
            trees += 1
        }
    }
    trees
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
        Ok(lines) => lines.map(|l| l.unwrap()).filter(|l| l != "").collect(),
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
    fn test_matrix() {
        let ns = numbers();
        assert_eq!(ns.len(), 4);
    }

    #[test]
    fn test_main() {
        main();
    }
}
