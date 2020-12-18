use std::fs::File;
use std::io::{self, prelude::*};

#[derive(Clone, Debug)]
enum Node {
    Num(i64),
    Op(char),
    Expr(Vec<Node>),
}

use Node::*;

pub fn main() {
    let ss = strings();
    let ress: Vec<_> = ss.iter().map(|s| solve(s)).collect();
    println!("RESULT: {:?} {:?}", ress, ress.iter().sum::<i64>());
}

fn solve(s: &str) -> i64 {
    println!("{:?}", s);
    let tokens = tokenize(s);
    println!("{:?}", tokens);
    let tree = parse(&tokens);
    println!("{:?}", tree);
    eval(&tree)
}

fn eval(n: &Node) -> i64 {
    match n {
        Num(i) => *i,
        Expr(ns) => eval_list(&ns),
        _ => unimplemented!(),
    }
}

fn eval_list(ns: &Vec<Node>) -> i64 {
    let mut i = 1;
    let mut a = eval(&ns[0]);
    while i < ns.len() {
        let b = eval(&ns[i + 1]);
        if let Op(o) = ns[i] {
            match o {
                '+' => a += b,
                '*' => {
                    a *= eval_list(&ns[i + 1..].to_vec());
                    i = ns.len();
                }
                _ => unimplemented!(),
            }
        }
        i += 2;
    }
    a
}

fn parse(chars: &Vec<char>) -> Node {
    if chars.len() == 1 {
        return parse_char(chars[0]);
    }
    let mut i = 0;
    let mut ns: Vec<Node> = Vec::new();
    while i < chars.len() {
        if chars[i] != '(' {
            ns.push(parse_char(chars[i]));
        } else {
            i += 1;
            let s = i;
            let mut pc = 0;
            while chars[i] != ')' || pc > 0 {
                if chars[i] == '(' {
                    pc += 1
                }
                if chars[i] == ')' {
                    pc -= 1
                }
                i += 1;
            }
            let v = &chars[s..i].to_vec();
            ns.push(parse(v));
        }
        i += 1;
    }
    Expr(ns)
}

fn parse_char(c: char) -> Node {
    match c.to_digit(10) {
        Some(n) => Num(n as i64),
        None => Op(c),
    }
}

fn tokenize(s: &str) -> Vec<char> {
    s.chars().filter(|&c| c != ' ').collect()
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
