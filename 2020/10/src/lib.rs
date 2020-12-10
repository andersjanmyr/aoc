use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*};

pub fn main() {
    let mut ns = numbers();
    ns.sort();
    let combs: Vec<Vec<i32>> = combinations(&ns.to_vec());

    println!("\n{:?} {:?}", combs, combs.len());
    // let (ones, threes) = diffs(&ns);
    // println!("{:?} {:?} {:?}", ones, threes, ones * threes);

    // let (v, i) = find(&ns, 25);
    // println!("{:?} {:?} {:?}", ns, i, v);
    // let (i, j, min, max) = find_first_sum(&ns, v);
    // println!("{:?} {:?} {:?} {:?} {:?}", i, j, min, max, min + max);
}

fn combinations(ns: &Vec<i32>) -> Vec<Vec<i32>> {
    let high = ns[ns.len() - 1];
    let mut v = ns.to_vec();
    v.push(high + 3);
    let mut queue: Vec<(i32, Vec<i32>)> = vec![(0, ns.to_vec())];
    let mut done: HashMap<(i32, Vec<i32>), i32> = HashMap::new();
    let mut combs: Vec<Vec<i32>> = Vec::new();
    while !queue.is_empty() {
        let (n, ns) = queue.remove(0);
        let key = (n, ns.to_vec());
        *done.entry(key).or_insert(0) += 1;
        let pvs = possible_values_for(n, &ns);
        for pv in pvs {
            if pv.0 == high {
                let v = ns.to_vec();
                combs.push(v);
            } else {
                if !done.contains_key(&pv) {
                    queue.push(pv);
                }
            }
        }
        // println!("{:?} {:?} {:?}", n, ns, queue.len(),);
    }

    println!("{:?} {:?} {:?}", done, done.len(), queue.len(),);
    combs
}

fn possible_values_for(a: i32, ns: &Vec<i32>) -> Vec<(i32, Vec<i32>)> {
    let mut pvs: Vec<(i32, Vec<i32>)> = Vec::new();
    let mut v = ns.to_vec();
    for i in 0..ns.len() {
        let n = ns[i];
        if is_valid(a, n) {
            v.remove(0);
            pvs.push((n, v.to_vec()));
        } else {
            break;
        }
    }
    pvs
}

fn is_valid(a: i32, b: i32) -> bool {
    let diff = (a - b).abs();
    diff >= 1 && diff <= 3
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
    fn test_bin_part() {
        let v = vec![1, 4, 5, 6, 7];
        let pvs = combinations(&v);
        println!("{:?}", pvs);
    }

    #[test]
    #[ignore]
    fn test_possible_values_for() {
        // let v = vec![4, 5, 6, 7];
        // let pvs = possible_values_for(3, &v);
        // println!("{:?}", pvs);
    }

    #[test]
    fn test_main() {
        main();
    }
}
