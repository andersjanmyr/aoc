use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*};

#[derive(Debug)]
struct Rule {
    name: String,
    e1: (i32, i32),
    e2: (i32, i32),
}

impl Rule {
    fn new(name: String, expr1: String, expr2: String) -> Self {
        Rule {
            name,
            e1: Self::parse_expr(expr1),
            e2: Self::parse_expr(expr2),
        }
    }

    fn parse_expr(e: String) -> (i32, i32) {
        let is: Vec<i32> = e.split("-").map(|s| s.parse::<i32>().unwrap()).collect();
        (is[0], is[1])
    }

    fn satisfies_rule(&self, n: i32) -> bool {
        (n >= self.e1.0 && n <= self.e1.1) || (n >= self.e2.0 && n <= self.e2.1)
    }
}

pub fn main() {
    let gs = groups();
    // let buses: Vec<(i32, i32)> = ls[1]
    //     .split(',')
    //     .enumerate()
    //     .filter_map(|(i, l)| l.parse().ok().map(|b| (i as i32, b)))
    //     .collect();
    let result = solve(&gs);
    println!("{:?}", result);
}

fn solve(gs: &Vec<Vec<String>>) -> usize {
    let rules = parse_rules(&gs[0]);
    println!("{:?}", rules);
    let ticket = parse_ticket(&gs[1]);

    let tickets = parse_tickets(&gs[2]);
    let valid = valid_tickets(&rules, &tickets);

    let ts = &valid;
    let iss: Vec<_> = (0..ts[0].len())
        .map(|i| ts.iter().map(|t| t[i]).collect::<Vec<i32>>())
        .collect();
    // println!("iss: {:?}", iss);

    let mut map: HashMap<String, Vec<usize>> = HashMap::new();
    for r in rules {
        for (i, is) in iss.iter().enumerate() {
            if is.iter().all(|i| r.satisfies_rule(*i)) {
                println!("i: {:?} {:?}", i, r.name);
                let v = map.entry(r.name.to_string()).or_insert(vec![i]);
                v.push(i);
                v.dedup();
            }
        }
    }
    println!("map: {:?}", map);
    let mut entries: Vec<(&str, Vec<usize>)> = Vec::new();
    for (k, v) in &map {
        entries.push((k, v.clone()));
    }
    entries.sort_by(|(_, a), (_, b)| a.len().partial_cmp(&b.len()).unwrap());
    println!("entries: {:?}", entries);
    for i in 0..entries.len() {
        let a = entries[i].1[0];
        for j in i + 1..entries.len() {
            let b = entries[j].1.iter().filter(|&n| n != &a).cloned().collect();
            entries[j].1 = b;
        }
    }
    println!("entries: {:?}", entries);
    let deps: Vec<_> = entries
        .iter()
        .filter(|(s, _)| s.starts_with("departure"))
        .collect();
    println!("deps: {:?}", deps);
    println!("ticket: {:?}", ticket);
    deps.iter().fold(1, |a, (_, v)| a * ticket[v[0]])
}

fn valid_tickets(rules: &Vec<Rule>, tickets: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    tickets
        .iter()
        .filter(|&t| valid_ticket(rules, t))
        .cloned()
        .collect()
}

fn valid_ticket(rules: &Vec<Rule>, ticket: &Vec<i32>) -> bool {
    ticket
        .iter()
        .all(|n| !rules.iter().all(|r| !r.satisfies_rule(*n)))
}

fn parse_rules(ss: &Vec<String>) -> Vec<Rule> {
    ss.iter().map(|l| parse_rule(&l)).collect()
}

fn parse_rule(s: &str) -> Rule {
    let parts: Vec<&str> = s.split(":").collect();
    let name = parts[0].to_string();
    let parts: Vec<&str> = parts[1].split(" ").collect();
    let expr1 = parts[1].to_string();
    let expr2 = parts[3].to_string();
    Rule::new(name, expr1, expr2)
}

fn parse_tickets(ss: &Vec<String>) -> Vec<Vec<i32>> {
    ss[1..]
        .iter()
        .map(|s| s.split(",").map(|s| s.parse::<i32>().unwrap()).collect())
        .collect()
}

fn parse_ticket(ss: &Vec<String>) -> Vec<usize> {
    ss[1].split(",").map(|s| s.parse().unwrap()).collect()
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
