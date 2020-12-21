use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, prelude::*};

#[derive(Clone, Debug)]
struct Tile {
    num: usize,
    rows: Vec<Vec<char>>,
    edges: Vec<Vec<char>>,
}

impl Tile {
    fn matches(&self, t: &Tile) -> bool {
        let (ok, _, _) = self.match_style(t);
        ok
    }

    fn match_style(&self, t: &Tile) -> (bool, usize, bool) {
        for e in &self.edges {
            for i in 0..4 {
                let f = &t.edges[i];
                if *e == *f {
                    return (true, i, false);
                }
                let mut rev = f.clone();
                rev.reverse();
                if *e == *rev {
                    return (true, i, true);
                }
            }
        }
        (false, 0, false)
    }

    fn convert(&self, rot: usize, flip: bool) -> Self {
        let mut out = Self {
            num: self.num,
            edges: self.edges.clone(),
            rows: self.rows.clone(),
        };
        for i in 0..rot {
            out = out.rotate();
        }
        if flip {
            let mut rows = out.rows.clone();
            rows.each
        }else {
            out
        }
    }

    fn rotate(&self) -> Self {
        let mut edges = self.edges.clone();
        edges.rotate_right(1);
        let mut rows = self.rows.clone();
        let len = self.rows.len();
        for i in 0..len {
            for j in 0..len {
                rows[j][len - i - 1] = self.rows[i][j];
            }
        }
        Self {
            num: self.num,
            edges,
            rows,
        }
    }

    fn to_string(&self) -> String {
        self.rows
            .iter()
            .map(|r| r.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

pub fn main() {
    let gs = groups();
    println!("Groups: {:?}", gs.len());
    let tiles: Vec<_> = gs.iter().map(parse_tile).collect();
    println!("{:?} {:?}", tiles, tiles.len());
    let mut map = HashMap::new();
    for i in 0..tiles.len() - 1 {
        for j in i + 1..tiles.len() {
            if tiles[i].matches(&tiles[j]) {
                let v = map.entry(tiles[i].num).or_insert(Vec::new());
                v.push(&tiles[j].num);
                let v = map.entry(tiles[j].num).or_insert(Vec::new());
                v.push(&tiles[i].num);
            }
        }
    }
    println!("{:?}", map);
    let corners: Vec<_> = map
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(k, _)| *k)
        .collect();
    println!("{:?}", corners);
    println!("{:?}", corners.iter().product::<usize>());
    let mut ts = tiles.clone();
    let i = ts.iter().position(|t| t.num == corners[0]).unwrap();
    let mut todos = vec![(ts.remove(i), 0, false)];
    while todos.len() > 0 {
        let mut item = todos.pop().unwrap();
        println!("{:?} {:?} {:?}", item.0.num, item.1, item.2);
        let cur = item.0.convert(item.1, item.2);
        println!("{}", item.0.to_string());
        println!("");
        println!("{}", cur.to_string());
        println!("");
        for num in map.get(&cur.num).unwrap() {
            let oi = ts.iter().position(|t| t.num == **num);
            if let Some(i) = oi {
                let neigh = ts.remove(i);
                let (ok, rot, flip) = cur.match_style(&neigh);
                todos.insert(0, (neigh, rot, flip));
            }
        }
    }
}

fn parse_tile(ts: &Vec<String>) -> Tile {
    let one = ts[0][5..9].to_string();
    let num = one.parse::<usize>().unwrap();
    let top = ts[1].chars().collect::<Vec<_>>();
    let right = ts[1..]
        .iter()
        .map(|s| s.chars().last().unwrap())
        .collect::<Vec<_>>();
    let mut bottom = ts.last().unwrap().chars().collect::<Vec<_>>();
    bottom.reverse();
    let mut left = ts[1..]
        .iter()
        .map(|s| s.chars().next().unwrap())
        .collect::<Vec<_>>();
    left.reverse();
    Tile {
        num,
        rows: ts[1..].iter().map(|s| s.chars().collect()).collect(),
        edges: vec![top, right, bottom, left],
    }
}

fn vec_to_usize(cs: Vec<char>) -> usize {
    let s: String = cs
        .iter()
        .map(|c| match c {
            '#' => '1',
            _ => '0',
        })
        .collect();
    usize::from_str_radix(&s, 2).unwrap()
}

fn tokenize(s: &str) -> Vec<char> {
    s.chars().filter(|&c| c != ' ').collect()
}

fn numbers() -> Vec<i64> {
    let lines = strings();
    lines.iter().map(|s| s.parse::<i64>().unwrap()).collect()
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
