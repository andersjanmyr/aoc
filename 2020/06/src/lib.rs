use std::fs::File;
use std::io::{self, prelude::*};

pub fn main() {
    let ls = strings();

    let mut max = 0;
    let mut seats = Vec::new();
    for l in ls {
        let (r, c, id) = calc_seat(&l);
        seats.push(id);
        if id > max {
            max = id;
        }
    }
    println!("MAX: {:?}", max);
    seats.sort();
    for i in 0..seats.len() - 1 {
        if seats[i] + 1 != seats[i + 1] {
            println!("Seat: {:?}", seats[i] + 1);
        }
    }
}

fn calc_seat(s: &str) -> (i32, i32, i32) {
    let r = bin_part(0, 127, &s[0..7]);
    let c = bin_part(0, 7, &s[7..]);
    (r, c, r * 8 + c)
}

fn bin_part(low: i32, high: i32, s: &str) -> i32 {
    let (l, _) = s.chars().fold((low, high), |(l, h), c| match c {
        'F' | 'L' => next_range(l, h, true),
        'B' | 'R' => next_range(l, h, false),
        _ => panic!(""),
    });
    l
}

fn next_range(low: i32, high: i32, choose_low: bool) -> (i32, i32) {
    let mid = (low + high) / 2;
    if choose_low {
        (low, mid)
    } else {
        (mid + 1, high)
    }
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
    fn test_bin_part() {
        let l = bin_part(0, 127, "FBFBBFF");
        assert_eq!(l, 44);
        let l = bin_part(0, 7, "RLR");
        assert_eq!(l, 5);
    }

    #[test]
    #[ignore]
    fn test_range() {
        let (l, p) = next_range(0, 127, false);
        assert_eq!((l, p), (64, 127));
        let (l, p) = next_range(0, 127, true);
        assert_eq!((l, p), (0, 63));
    }

    #[test]
    fn test_main() {
        main();
    }
}
