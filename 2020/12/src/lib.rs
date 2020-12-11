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
    }
    print_grid(&next);
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
            let adj = adjacent(grid, r, c);
            if grid[r][c] == 'L' && adj == 0 {
                copy[r][c] = '#';
            }
            if grid[r][c] == '#' && adj >= 5 {
                copy[r][c] = 'L';
            }
        }
    }
    copy
}

fn adjacent(grid: &Vec<Vec<char>>, r: usize, c: usize) -> i32 {
    let mut count = 0;
    for line in adjacents(grid.len(), grid[0].len(), r, c) {
        for (i, j) in line {
            if grid[i][j] == '#' {
                count += 1;
                break;
            }
            if grid[i][j] == 'L' {
                break;
            }
        }
    }
    // println!("{:?} {:?} {:?}", r, c, vs);
    count
}

fn get_coordinates(
    r: i32,
    c: i32,
    rlim: i32,
    clim: i32,
    rdelta: i32,
    cdelta: i32,
) -> Vec<(usize, usize)> {
    let mut v: Vec<(usize, usize)> = Vec::new();
    let mut i = r as i32;
    let mut j = c as i32;
    while i != rlim as i32 && j != clim as i32 {
        v.push((i as usize, j as usize));
        i += rdelta;
        j += cdelta;
    }
    v
}

fn adjacents(rlen: usize, clen: usize, r: usize, c: usize) -> Vec<Vec<(usize, usize)>> {
    let mut vs: Vec<Vec<(usize, usize)>> = Vec::new();
    let rmin: i32 = r as i32 - 1;
    let cmin: i32 = c as i32 - 1;
    let rmax = r as i32 + 1;
    let cmax = c as i32 + 1;

    vs.push(get_coordinates(rmin, cmin, -1, -1, -1, -1));
    vs.push(get_coordinates(rmin, c as i32, -1, -1, -1, 0));
    vs.push(get_coordinates(rmin, cmax, -1, clen as i32, -1, 1));
    vs.push(get_coordinates(r as i32, cmin, -1, -1, 0, -1));
    vs.push(get_coordinates(r as i32, cmax, -1, clen as i32, 0, 1));
    vs.push(get_coordinates(rmax, cmin, rlen as i32, -1, 1, -1));
    vs.push(get_coordinates(rmax, c as i32, rlen as i32, -1, 1, 0));
    vs.push(get_coordinates(rmax, cmax, rlen as i32, clen as i32, 1, 1));

    vs
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
