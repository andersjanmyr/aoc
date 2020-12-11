use std::fs::File;
use std::io::{self, prelude::*};

pub fn main() {
    let ls = strings();
    let mut grid: Vec<Vec<char>> = ls.iter().map(|s| s.chars().collect()).collect();
    print_grid(&grid);

    let mut next = simulate(&grid);
    print_grid(&next);
    while next != grid {
        grid = next;
        next = simulate(&grid);
        print_grid(&next);
    }
    println!("{:?}", count(&grid));
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for r in grid {
        let s: String = r.into_iter().collect();
        println!("{:?}", s);
    }
    println!("");
}

fn count(grid: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == '#' {
                count += 1;
            }
        }
    }
    count
}

fn simulate(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut copy = grid.to_vec();
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            let adj = adjacent(grid, r, c, '#');
            if grid[r][c] == 'L' && adj == 0 {
                copy[r][c] = '#';
            }
            if grid[r][c] == '#' && adj >= 4 {
                copy[r][c] = 'L';
            }
        }
    }
    copy
}

fn adjacent(grid: &Vec<Vec<char>>, r: usize, c: usize, seat: char) -> i32 {
    let mut count = 0;
    for (i, j) in adjacents(grid.len(), grid[0].len(), r, c) {
        if grid[i][j] == seat {
            count += 1;
        }
    }
    count
}

fn adjacents(rlen: usize, clen: usize, r: usize, c: usize) -> Vec<(usize, usize)> {
    let mut v: Vec<(usize, usize)> = Vec::new();
    let rmin: i32 = r as i32 - 1;
    let cmin: i32 = c as i32 - 1;
    let rmax = r + 1;
    let cmax = c + 1;
    if rmin >= 0 && cmin >= 0 {
        v.push((rmin as usize, cmin as usize));
    }
    if rmin >= 0 {
        v.push((rmin as usize, c));
    }
    if rmin >= 0 && cmax < clen {
        v.push((rmin as usize, cmax));
    }
    if cmin >= 0 {
        v.push((r, cmin as usize));
    }
    if cmax < clen {
        v.push((r, cmax));
    }
    if rmax < rlen && cmin >= 0 {
        v.push((rmax, cmin as usize));
    }
    if rmax < rlen {
        v.push((rmax, c));
    }
    if rmax < rlen && cmax < clen {
        v.push((rmax, cmax));
    }
    // println!("{:?} {:?} {:?}", r, c, v);
    v
}

fn find_first_sum(ns: &Vec<i64>, s: i64) -> (usize, usize, i64, i64) {
    for i in 0..ns.len() {
        for j in i + 1..ns.len() {
            let mut v = ns[i..j].to_vec();
            let sum = v.iter().fold(0, |a, i| a + i);
            if sum == s {
                v.sort();
                return (i, j - 1, v[0], v[v.len() - 1]);
            }
            if sum > s {
                break;
            }
        }
    }
    panic!("");
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

fn numbers() -> Vec<i64> {
    let lines = strings();
    lines.iter().map(|s| s.parse::<i64>().unwrap()).collect()
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
