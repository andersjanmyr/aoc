use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;

fn main() {
    let card_pub_key = 12578151;
    let door_pub_key = 5051300;
    let card_loop_size = solve(card_pub_key);
    println!("card_loop_size: {}", card_loop_size);
    let door_loop_size = solve(door_pub_key);
    println!("door_loop_size: {}", door_loop_size);

    println!("{:?}", transform(card_pub_key, door_loop_size));
    println!("{:?}", transform(door_pub_key, card_loop_size));
    // let mut cups = Cups::new(1_000_000, v.clone());
    // for _ in 0..10_000_000 {
    //     cups.step()
    // }
    // println!("Star 2: {}", cups.next_after(1).take(2).product::<usize>());
}

fn transform(pub_key: usize, loop_size: usize) -> usize {
    let mut value = 1;

    for i in 0..loop_size {
        value = (value * pub_key) % 20201227;
    }
    value
}

fn solve(pub_key: usize) -> usize {
    let mut loop_size = 0;
    let mut value = 1;

    while value != pub_key {
        loop_size += 1;
        value = (value * 7) % 20201227;
    }
    loop_size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
