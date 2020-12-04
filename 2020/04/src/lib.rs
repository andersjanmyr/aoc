use regex::Regex;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, prelude::*};

pub fn main() {
    let ps = passports();
    let valids: Vec<Passport> = ps.into_iter().filter(|p| p.is_valid()).collect();
    println!("{:?}, {:?}", valids, valids.len());
}

// ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
// byr:1937 iyr:2017 cid:147 hgt:183cm
#[derive(Debug, Clone)]
pub struct Passport {
    ecl: String,
    pid: String,
    eyr: String,
    hcl: String,
    byr: String,
    iyr: String,
    cid: String,
    hgt: String,
}

fn get(s: &str, name: &str) -> String {
    let pairs = s.split(" ");
    for p in pairs {
        let v: Vec<&str> = p.split(":").collect();
        if v[0] == name {
            return v[1].to_string();
        }
    }
    return "".to_string();
}

impl Passport {
    pub fn new(pairs: &str) -> Self {
        println!("{:?}", pairs);
        let p = Passport {
            ecl: get(pairs, "ecl"),
            pid: get(pairs, "pid"),
            eyr: get(pairs, "eyr"),
            hcl: get(pairs, "hcl"),
            byr: get(pairs, "byr"),
            iyr: get(pairs, "iyr"),
            cid: get(pairs, "cid"),
            hgt: get(pairs, "hgt"),
        };
        p
    }
    pub fn is_valid(&self) -> bool {
        Self::is_valid_num(&self.byr, 1920, 2020)
            && Self::is_valid_num(&self.iyr, 2010, 2020)
            && Self::is_valid_num(&self.eyr, 2020, 2030)
            && self.is_valid_hgt()
            && self.is_valid_hcl()
            && self.is_valid_ecl()
            && self.is_valid_pid()
    }

    pub fn is_valid_num(s: &str, min: i32, max: i32) -> bool {
        match s.parse::<i32>() {
            Ok(i) => i >= min && i <= max,
            Err(_) => false,
        }
    }

    pub fn is_valid_hgt(&self) -> bool {
        if self.hgt == "" {
            return false;
        }
        let len = self.hgt.len() - 2;
        if self.hgt.ends_with("in") {
            Self::is_valid_num(&self.hgt[..len], 59, 76)
        } else if self.hgt.ends_with("cm") {
            Self::is_valid_num(&self.hgt[..len], 150, 193)
        } else {
            false
        }
    }

    pub fn is_valid_hcl(&self) -> bool {
        let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        re.is_match(&self.hcl)
    }

    pub fn is_valid_ecl(&self) -> bool {
        let re = Regex::new(r"^(amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth)$").unwrap();
        re.is_match(&self.ecl)
    }
    pub fn is_valid_pid(&self) -> bool {
        let re = Regex::new(r"^[0-9]{9}$").unwrap();
        re.is_match(&self.pid)
    }
}

fn passports() -> Vec<Passport> {
    let mut ps = Vec::new();
    let ls = strings();
    let mut pairs = "".to_string();
    for l in ls {
        if l == "" {
            let p = Passport::new(&pairs);
            ps.push(p);
            pairs = "".to_string();
        }
        pairs.push_str(" ");
        pairs.push_str(&l);
    }
    let p = Passport::new(&pairs);
    ps.push(p);
    ps
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
    fn test_matrix() {
        let ns = numbers();
        assert_eq!(ns.len(), 4);
    }

    #[test]
    fn test_main() {
        main();
    }
}
