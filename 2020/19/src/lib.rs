use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*};

#[derive(Clone, Debug)]
pub enum Rule {
    Term(String),
    Single(Vec<i32>),
    Double(Vec<i32>, Vec<i32>),
}

use Rule::*;

pub fn main() {
    let gs = groups();
    let rules = parse_rules(&gs[0]);
    println!("{:?} {:?}", rules, rules.len());
    let mut cache = HashMap::new();
    let expanded = expand(&rules, 0, &mut cache);
    println!("{:?}", expanded.len());
    let max = gs[1].iter().map(|s| s.len()).max().unwrap();
    println!("max: {:?}", max);
    let (nostars, stars) = expand_stars(&expanded, &cache);
    println!("Exp {:?} {:?}", stars.len(), nostars.len());
    let mut matching = check(&gs[1], &nostars);
    println!("Matching {:?}", matching.len());
    let mut matching_stars = reg_check(&gs[1], &stars);
    println!(
        "Matching_stars {:?} {:?}",
        matching_stars,
        matching_stars.len()
    );
    matching.append(&mut matching_stars);
    matching.sort();
    matching.dedup();
    println!("Total {:?} {:?}", matching, matching.len());
}

fn reg_check(needles: &Vec<String>, haystack: &Vec<String>) -> Vec<String> {
    let mut i = 0;
    println!("{:?}", haystack.len());
    let rs: Vec<_> = haystack
        .into_iter()
        .map(|s| {
            i += 1;
            if i % 1000 == 0 {
                println!("rc{:?}", i);
            }
            Regex::new(&format!("^{}$", s)).unwrap()
        })
        .collect();
    println!("Regex {:?}", rs.len());
    let matches: Vec<String> = needles
        .into_iter()
        .filter(|n| {
            rs.iter().any(|r| {
                let ok = r.is_match(n);
                if ok {
                    println!("{:?} {:?}", r, n);
                }
                ok
            })
        })
        .cloned()
        .collect();
    matches
}

fn check(needles: &Vec<String>, haystack: &Vec<String>) -> Vec<String> {
    let set: HashSet<&String> = haystack.iter().collect();
    println!("Check {:?}", needles.len());
    needles
        .iter()
        .filter(|&n| set.contains(&n))
        .cloned()
        .collect()
}

fn expand_stars(
    haystack: &Vec<String>,
    cache: &HashMap<i32, Vec<String>>,
) -> (Vec<String>, Vec<String>) {
    let eight: Vec<String> = cache
        .get(&8)
        .unwrap()
        .iter()
        .filter(|s| s.contains("*"))
        .cloned()
        .collect();
    let eleven: Vec<String> = cache
        .get(&11)
        .unwrap()
        .iter()
        .filter(|s| s.contains("+"))
        .cloned()
        .collect();
    let nostars: Vec<_> = haystack
        .iter()
        .filter(|&h| !h.contains("*") && !h.contains("+"))
        .cloned()
        .collect();
    let stars: Vec<_> = haystack
        .into_iter()
        .filter(|&h| h.contains("*") || h.contains("+"))
        .collect();
    let mut res = Vec::new();
    let mut res2 = Vec::new();
    let exps: Vec<_> = expand_star(&eleven, "+");
    for h in stars {
        if h.contains("+") {
            let v = expand_string(h, &exps, "+");
            for s in &v {
                res2.push(s.to_string());
            }
        } else {
            res2.push(h.to_string());
        }
    }
    let exps: Vec<_> = expand_star(&eight, "*");
    for h in &res2 {
        if h.contains("*") {
            let v = expand_string(h, &exps, "*");
            for s in &v {
                res.push(s.to_string());
            }
        } else {
            res.push(h.to_string());
        }
    }
    (nostars, res)
}

fn expand_string(s: &str, exps: &Vec<String>, mark: &str) -> Vec<String> {
    exps.iter().map(|e| s.replace(mark, e)).collect()
}

fn expand_star(ss: &Vec<String>, mark: &str) -> Vec<String> {
    let pairs: Vec<_> = ss
        .into_iter()
        .map(|s| {
            let parts: Vec<_> = s.split(mark).collect();
            let p0 = parts[0];
            let p1 = if parts.len() > 1 { parts[1] } else { "" };
            (p0, p1)
        })
        .collect();
    if pairs[0].1 == "" {
        let s = pairs
            .into_iter()
            .map(|(p, _)| p)
            .collect::<Vec<_>>()
            .join("|");
        return vec![format!("({})+", s)];
    }

    let s1 = pairs.iter().map(|&(p, _)| p).collect::<Vec<_>>().join("|");
    let s2 = pairs
        .into_iter()
        .map(|(_, p)| p)
        .collect::<Vec<_>>()
        .join("|");
    let patterns: Vec<_> = (1..3)
        .map(|i| format!("({0}){{{2}}}({1}){{{2}}}", s1, s2, i))
        .collect();
    patterns
}

fn expand(
    rules: &HashMap<i32, Rule>,
    i: i32,
    cache: &mut HashMap<i32, Vec<String>>,
) -> Vec<String> {
    println!("Expand: {:?}", i);
    if let Some(ss) = cache.get(&i) {
        println!("Cache {:?}", i);
        return ss.clone();
    }
    let r = rules.get(&i).unwrap();
    match r {
        Term(s) => {
            let ss = vec![s.to_string()];
            cache.insert(i, ss.clone());
            ss
        }
        Single(v) => {
            let ss = expand_vec(rules, v, cache);
            cache.insert(i, ss.clone());
            ss
        }
        Double(v1, v2) => {
            let mut ss = expand_vec(rules, v1, cache);
            ss.append(&mut expand_vec(rules, v2, cache));
            cache.insert(i, ss.clone());
            ss
        }
    }
}

fn expand_vec(
    rules: &HashMap<i32, Rule>,
    v: &Vec<i32>,
    cache: &mut HashMap<i32, Vec<String>>,
) -> Vec<String> {
    let mut sss: Vec<_> = v.iter().map(|&i| expand(rules, i, cache)).collect();
    let res = expand_strings(&mut sss, vec!["".to_string()]);
    res
}

fn expand_strings(sss: &mut Vec<Vec<String>>, current: Vec<String>) -> Vec<String> {
    println!("SSS{:?}", sss.len());

    if let Some(ss) = sss.pop() {
        println!("SS: {:?}", ss.len());
        let mut res = Vec::new();
        for el in current {
            for s in &ss {
                res.push(format!("{}{}", s, el));
            }
        }
        expand_strings(sss, res)
    } else {
        current
    }
}

fn parse_rules(ss: &Vec<String>) -> HashMap<i32, Rule> {
    ss.iter().map(|s| parse_line(&s)).collect()
}

fn parse_line(s: &str) -> (i32, Rule) {
    let ss: Vec<&str> = s.split(":").collect();
    let num: i32 = ss[0].parse::<i32>().unwrap();
    let val = ss[1].trim();
    if val.contains("\"") {
        return (num, Term(val.replace("\"", "")));
    }
    let ss: Vec<&str> = val.split("|").collect();
    println!("{:?}", ss);
    if ss.len() == 1 {
        return (num, Single(parse_nums(ss[0])));
    } else {
        return (num, Double(parse_nums(ss[0]), parse_nums(ss[1])));
    }
}

fn parse_nums(s: &str) -> Vec<i32> {
    s.trim()
        .split(" ")
        .map(|s| s.parse::<i32>().unwrap())
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
