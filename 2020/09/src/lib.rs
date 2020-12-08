use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*};

#[derive(Clone, Debug)]
pub struct Instr {
    name: String,
    num: i32,
}

pub fn main() {
    let instrs = parse_instr();

    let (i, acc, ok) = exec(&instrs);
    println!("{:?} {:?} {:?}", i, acc, ok);
    for i in 0..instrs.len() {
        let instr = &instrs[i];
        if instr.name == "jmp" {
            let mut new_instrs = instrs.clone();
            new_instrs[i] = Instr {
                name: "nop".to_string(),
                num: instr.num,
            };
            let (i, acc, ok) = exec(&new_instrs);
            if ok {
                println!("{:?} {:?} {:?}", i, acc, ok);
            }
        }
    }
}

fn exec(instrs: &Vec<Instr>) -> (i32, i32, bool) {
    let len = instrs.len() as i32;
    let mut acc = 0;
    let mut set = HashSet::new();
    let mut i: i32 = 0;
    while !set.contains(&i) && i < len {
        set.insert(i);
        let instr = &instrs[i as usize];
        match instr.name.as_str() {
            "nop" => i += 1,
            "acc" => {
                i += 1;
                acc += instr.num
            }
            "jmp" => i += instr.num,
            v => println!("{:?}", v),
        }
    }
    (i, acc, i == len)
}

fn parse_instr() -> Vec<Instr> {
    let ls = strings();
    ls.iter().map(|l| parse_line(&l)).collect()
}

fn parse_line(l: &str) -> Instr {
    let parts: Vec<&str> = l.split(" ").collect();
    let name = parts[0];
    let num = parts[1].parse::<i32>().unwrap();
    Instr {
        name: name.to_string(),
        num: num,
    }
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
