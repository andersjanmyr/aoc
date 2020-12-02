use std::fs::File;
use std::io::{self, prelude::*};

pub fn main() {
    let e = entries();
    let valid: Vec<&(i32, i32, char, String)> = e
        .iter()
        .filter(|(a, b, c, d)| {
            let mut n = 0;
            if nth(d, *a) == *c {
                n += 1;
            }
            if nth(d, *b) == *c {
                n += 1;
            }
            n == 1
        })
        .collect();
    println!("{:?} {:?}", valid, valid.len());
}

fn nth(s: &String, i: i32) -> char {
    match s.chars().nth(i as usize - 1) {
        Some(c) => c,
        None => {
            println!("{:?}", s);
            'X'
        }
    }
}

fn entries() -> Vec<(i32, i32, char, String)> {
    let lines = strings();
    lines.iter().map(|s| parse_line(s.to_string())).collect()
}

// l = "1-3 a: abcde"
fn parse_line(l: String) -> (i32, i32, char, String) {
    let parts = l.split(" ").collect::<Vec<&str>>();
    let nums = parts[0].split("-").collect::<Vec<&str>>();
    let a = nums[0].parse::<i32>().unwrap();
    let b = nums[1].parse::<i32>().unwrap();
    let c = parts[1].chars().next().unwrap();
    let d = parts[2];
    (a, b, c, d.to_string())
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
    fn test_matrix() {
        let ns = main();
    }

    #[test]
    #[ignore]
    fn test_main() {
        assert_eq!(
            (1, 3, 'a', "abcde".to_string()),
            parse_line("1-3 a: abcde".to_string())
        );
    }
}
