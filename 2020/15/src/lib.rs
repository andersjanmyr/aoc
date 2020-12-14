use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*};

#[derive(Debug)]
enum Rule {
    Mask(String),
    Mem { pos: usize, val: usize },
}
use Rule::*;

pub fn main() {
    let ls = strings();
    let rules = parse_rules(ls);
    println!("{:?}", rules);
    let result = simulate(rules);
    println!("{:?}", result);
}

fn simulate(rules: Vec<Rule>) -> usize {
    let mut mem: HashMap<usize, usize> = HashMap::new();
    let mut mask: String = "".to_string();
    for rule in rules {
        match rule {
            Mask(m) => mask = m,
            Mem { pos, val } => {
                let ps = mask_value(pos, mask.to_string());
                for p in ps {
                    mem.insert(p, val);
                }
            }
        }
    }
    mem.values().sum()
}

fn mask_value(pos: usize, mask: String) -> Vec<usize> {
    let mut bs: Vec<_> = format!("{:036b}", pos).chars().collect();
    let cs = mask.chars();
    for (i, c) in cs.enumerate() {
        match c {
            '0' => (),
            '1' => bs[i] = '1',
            _ => bs[i] = 'X',
        }
    }
    let mut v = vec![bs];
    expand_pos(&mut v);
    v.iter()
        .map(|cs| {
            let s: String = cs.into_iter().collect();
            usize::from_str_radix(&s, 2).unwrap()
        })
        .collect()
}

fn expand_pos(v: &mut Vec<Vec<char>>) {
    let mut old_len = 0;
    while v.len() != old_len {
        old_len = v.len();
        let cs = v.pop().unwrap();
        let i = cs.iter().position(|&c| c == 'X');
        match i {
            Some(i) => {
                let mut c0 = cs.clone();
                c0[i] = '0';
                v.insert(0, c0);
                let mut c1 = cs.clone();
                c1[i] = '1';
                v.insert(0, c1);
            }
            None => v.push(cs),
        }
    }
}

fn parse_rules(ls: Vec<String>) -> Vec<Rule> {
    let rmem = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
    let rmask = Regex::new(r"^mask = (.+)$").unwrap();
    ls.iter()
        .map(|l| match rmask.captures(l) {
            Some(cap) => Mask(cap[1].to_string()),
            None => {
                let cap = rmem.captures(l).unwrap();
                Mem {
                    pos: cap[1].parse().unwrap(),
                    val: cap[2].parse().unwrap(),
                }
            }
        })
        .collect()
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
    fn test_bin_part() {}

    #[test]
    #[ignore]
    fn test_range() {}

    #[test]
    fn test_main() {
        main();
    }
}
