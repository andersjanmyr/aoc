use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt");
    let data = input.unwrap();
    let (p1, p2) = parse_input(&data);

    println!("{:?} {:?}", p1, p2);
    let mut set = HashSet::new();
    let (p1, p2) = play_game(p1, p2, &mut set);
    println!("{:?} {:?}", p1, p2);
    let score = if p1.len() > p2.len() {
        calc_score(p1)
    } else {
        calc_score(p2)
    };
    println!("{:?}", score);
}

fn calc_score(ps: Vec<usize>) -> usize {
    let mut sum = 0;
    for i in 0..ps.len() {
        let f = (ps.len() - i) as usize;
        sum += f * ps[i];
    }
    sum
}

fn play_game(
    p1: Vec<usize>,
    p2: Vec<usize>,
    set: &mut HashSet<(Vec<usize>, Vec<usize>)>,
) -> (Vec<usize>, Vec<usize>) {
    println!("Game: {:?}", set.len());
    let mut p1 = p1;
    let mut p2 = p2;
    while p1.len() > 0 && p2.len() > 0 {
        if set.contains(&(p1.clone(), p2.clone())) {
            println!("inf");
            return (p1.clone(), p2.clone());
        }
        set.insert((p1.clone(), p2.clone()));
        let c1 = p1.remove(0);
        let c2 = p2.remove(0);
        if c1 <= p1.len() && c2 <= p2.len() {
            let (r1, _) = play_game(p1.clone(), p2.clone(), set);
            if r1.len() > 0 {
                p1.push(c1);
                p1.push(c2);
            } else {
                p2.push(c2);
                p2.push(c1);
            }
        } else if c1 > c2 {
            p1.push(c1);
            p1.push(c2);
        } else {
            p2.push(c2);
            p2.push(c1);
        }
    }
    (p1, p2)
}

fn parse_input(s: &str) -> (Vec<usize>, Vec<usize>) {
    let v: Vec<Vec<usize>> = s
        .lines()
        .collect::<Vec<_>>()
        .split(|&l| l == "")
        .map(|lines| {
            lines[1..]
                .iter()
                .map(|l| l.parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    (v[0].clone(), v[1].clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
