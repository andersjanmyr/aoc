use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;

#[derive(Debug)]
struct Ring {
    v: Vec<usize>,
    len: usize,
}

impl Ring {
    fn new(v: Vec<usize>) -> Self {
        let len = v.len();
        Self { v, len }
    }

    fn at(&self, i: usize) -> usize {
        self.v[self.index(i)]
    }

    fn safe(&self, i: usize) -> usize {
        if i < 1 {
            i + self.len
        } else {
            i
        }
    }

    fn index(&self, ind: usize) -> usize {
        let mut i = ind - 1;
        if i < 0 {
            i + self.len
        } else {
            i
        }
    }

    fn rm(&mut self, from: usize, count: usize) -> Vec<usize> {
        let mut ps = (from..from + count)
            .map(|i| self.index(i))
            .enumerate()
            .collect::<Vec<_>>();
        ps.sort_by(|a, b| b.1.cmp(&a.1));
        let mut three = vec![0; count];
        for &(i, p) in &ps {
            let e = self.v.remove(p);
            three[i as usize] = e;
        }
        three
    }

    fn insert(&mut self, el: usize, slice: &Vec<usize>) {
        let mut i = ind + 1;
        if i < 0 {
            i += self.v.len()
        }
        self.v.splice(i..i, slice.clone());
    }
}

fn main() {
    let input = read_to_string("input.txt");
    let data = input.unwrap();
    let v = parse_input(&data);
    let mut ring = Ring::new(v);
    println!("{:?}", ring);

    for i in 1..=10 {
        mov(&mut ring, i);
    }
}

fn mov(ring: &mut Ring, cur: usize) {
    let cur_el = ring.at(cur);
    let mut three = ring.rm(cur + 1, 3);
    let mut dest = ring.at(cur_el - 1);
    println!("{:?}({:?}) {:?} {:?}", cur, cur_el, dest, three);
    ring.insert(dest, &three);
    println!("{:?}", ring);
    println!("");
}

fn parse_input(s: &str) -> Vec<usize> {
    s.trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|i| i as usize)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
