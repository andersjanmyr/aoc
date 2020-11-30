use std::fs::File;
use std::io::{self, prelude::*};

pub fn main() {
    println!("{:?}", numbers());
}

fn numbers() -> Vec<i32> {
    let lines = strings();
    lines
        .iter()
        .filter(|s| s.parse::<i32>().is_ok())
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn strings() -> Vec<String> {
    let lines = read_lines();
    match lines {
        Ok(lines) => lines.map(|l| l.unwrap()).collect(),
        Err(_) => Vec::new(),
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
    fn test_numbers() {
        let ns = numbers();
        assert_eq!(ns.len(), 4);
    }
}
