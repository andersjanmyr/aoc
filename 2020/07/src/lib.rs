use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*};

#[derive(Debug)]
pub struct Rule {
    name: String,
    has: Vec<ContainedRule>,
}

#[derive(Debug)]
pub struct ContainedRule {
    count: i32,
    name: String,
}

pub fn main() {
    let rules = parse_rules();
    println!("{:?}", rules);
    // let matching = traverse(&rules, "shiny gold");
    // println!("{:?} {:?}", matching, matching.len());
    let count = traverse2("shiny gold", &rules);
    println!("{:?}", count - 1);
}

fn parse_rules() -> Vec<Rule> {
    let ls = strings();
    ls.iter().map(|l| parse_line(&l)).collect()
}

fn parse_line(l: &str) -> Rule {
    let mut s = l.replace(",", "");
    s = s.replace(".", "");
    let parts: Vec<&str> = s.split(" ").collect();
    let name = format!("{} {}", parts[0], parts[1]);
    let mut has = Vec::new();
    for i in (4..parts.len()).step_by(4) {
        if parts[i] != "no" {
            let cr = ContainedRule {
                count: parts[i].parse::<i32>().unwrap(),
                name: format!("{} {}", parts[i + 1], parts[i + 2]),
            };
            has.push(cr);
        }
    }
    Rule {
        name: name,
        has: has,
    }
}

fn traverse(rules: &Vec<Rule>, bag: &str) -> Vec<String> {
    let mut set = HashSet::new();
    set.insert(bag.to_string());
    let mut len = 0;
    while len < set.len() {
        len = set.len();
        for r in rules {
            println!("{:?}", r);
            for cr in &r.has {
                if set.contains(&cr.name) {
                    println!("{:?}", cr.name);
                    set.insert(r.name.to_string());
                }
            }
        }
        println!("{:?}\n", set);
    }
    set.remove(bag);
    set.into_iter().collect()
}

fn traverse2(name: &str, rules: &Vec<Rule>) -> i32 {
    let rule = rules.iter().find(|r| r.name == name).unwrap();
    rule.has.iter().fold(1, |a, cr| {
        let c = a + (cr.count * traverse2(&cr.name, rules));
        c
    })
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
