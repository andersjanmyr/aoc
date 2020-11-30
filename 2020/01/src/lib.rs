use std::fs::File;
use std::io::{self, prelude::*};

pub fn main() {
    println!("{:?}", num_matrix());
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
        let ns = matrix();
        assert_eq!(ns.len(), 4);
    }

    #[test]
    fn test_main() {
        main();
    }
}
