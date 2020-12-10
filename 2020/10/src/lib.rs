use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*};

pub fn main() {
    let mut ns = numbers();
    ns.sort();
    let combs: Vec<Vec<i32>> = combinations(&ns.to_vec());

    println!("{:?} {:?}", combs.len(), combs.len());
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
    v.insert(0, 0);
    v.push(high + 3);
    let map = calc_possible_values(&v);
    let mut optionals = ns.to_vec();
    let mut set: HashSet<Vec<i32>> = HashSet::new();
    println!("OPT: {:?}", optionals);
    let mut seed = 12345;
    for (k, _) in map.iter() {
        let m = map[&k].to_vec();
        add_perms(&m, &mut set, &mut seed);
        seed += 1;
        if m.len() > 1 {
            optionals.retain(|n| *n != m[0]);
        }
    }
    let mut prod: i32 = 1;
    for m in optionals.iter() {
        let mut count = 0;
        for s in set.iter() {
            if s.contains(&m) {
                count += 1;
            }
        }
        println!("C: {:?} {:?}", m, count);
        prod *= count;
    }
    println!("NS: {:?}", ns);
    println!("OPT: {:?}", optionals);
    println!("Set: {:?}", set);
    println!("Prod: {:?}", prod);
    let mut combos = vec![vec![0]];
    combos
}

fn add_perms(ns: &Vec<i32>, set: &mut HashSet<Vec<i32>>, seed: &mut i32) {
    let mut v = ns.to_vec();
    println!("V: {:?}", v);
    if v.len() == 1 {
        set.insert(v.to_vec());
    }
    if v.len() == 2 {
        set.insert(v.to_vec());
        set.insert(v[0..1].to_vec());
        set.insert(v[1..2].to_vec());
    }
    if v.len() == 3 {
        set.insert(v.to_vec());
        set.insert(v[0..2].to_vec());
        set.insert(vec![v[0], v[2]]);
        set.insert(v[1..3].to_vec());
        set.insert(v[0..1].to_vec());
        set.insert(v[1..2].to_vec());
        set.insert(v[2..3].to_vec());
    }
}

fn calc_possible_values(ns: &Vec<i32>) -> HashMap<i32, Vec<i32>> {
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    for i in 0..ns.len() - 1 {
        let mut v = Vec::new();
        for j in i + 1..ns.len() {
            if !is_valid(ns[i], ns[j]) {
                break;
            }
            v.push(ns[j])
        }
        map.insert(ns[i], v);
    }
    for (k, x) in map.iter().sorted_by_key(|x| x.1) {
        println!("{:?},{:?}", k, x);
    }
    map
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
        let v = vec![4, 5, 6, 7];
        let pvs = possible_values_for(3, &v);
        println!("{:?}", pvs);
    }

    #[test]
    fn test_main() {
        main();
    }
}
