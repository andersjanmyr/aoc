use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt");
    let data = input.unwrap();
    let (p1, p2) = parse_input(&data);

    println!("{:?} {:?}", p1, p2);
    let (p1, p2) = play_game(p1, p2);
    println!("{:?} {:?}", p1, p2);
    let score = if p1.len() > p2.len() {
        calc_score(p1)
    } else {
        calc_score(p2)
    };
    println!("{:?}", score);
}

fn calc_score(ps: VecDeque<u8>) -> usize {
    let mut sum: usize = 0;
    for i in 0..ps.len() {
        let f = (ps.len() - i) as usize;
        sum += f * ps[i] as usize;
    }
    sum
}

fn play_game(mut p1: VecDeque<u8>, mut p2: VecDeque<u8>) -> (VecDeque<u8>, VecDeque<u8>) {
    let mut set = HashSet::new();
    while p1.len() > 0 && p2.len() > 0 {
        if !set.insert((p1.clone(), p2.clone())) {
            println!("inf {:?}", p1.len() + p2.len());
            return (p1, p2);
        }
        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();
        if c1 <= p1.len() as u8 && c2 <= p2.len() as u8 {
            let (r1, _) = play_game(
                p1.iter().take(c1 as usize).cloned().collect(),
                p2.iter().take(c2 as usize).cloned().collect(),
            );
            if r1.len() > 0 {
                p1.push_back(c1);
                p1.push_back(c2);
            } else {
                p2.push_back(c2);
                p2.push_back(c1);
            }
        } else if c1 > c2 {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }
    (p1, p2)
}

fn parse_input(s: &str) -> (VecDeque<u8>, VecDeque<u8>) {
    let gs = s.trim().split("\n\n").collect::<Vec<&str>>();
    (parse_cards(&gs[0]), parse_cards(&gs[1]))
}

fn parse_cards(s: &str) -> VecDeque<u8> {
    let mut iter = s.lines();
    iter.next();
    iter.map(|l| l.parse::<u8>().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
