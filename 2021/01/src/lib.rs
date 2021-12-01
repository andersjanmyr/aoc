use itertools::Itertools;

fn main() {
    let text = include_str!("../input.txt");
    let mut lines = text.split("\n").collect::<Vec<_>>();
    lines.pop();
    let nums = lines.iter().map(|s| s.parse::<i32>().unwrap());
    let tuples = nums.tuple_windows().collect::<Vec<(i32, i32, i32)>>();
    let sums = tuples.iter().map(|(a, b, c)| a + b + c);
    let tups = sums.tuple_windows().collect::<Vec<(i32, i32)>>();
    let incs: Vec<_> = tups.iter().filter(|(a, b)| a < b).collect();
    println!("{:?}", incs.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
