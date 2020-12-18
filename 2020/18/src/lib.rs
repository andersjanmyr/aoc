use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*};

pub fn main() {
    let mut gen: HashMap<(i32, i32, i32, i32), bool> = HashMap::new();
    let ss = strings();
    for (x, s) in ss.iter().enumerate() {
        for (y, c) in s.chars().enumerate() {
            if c == '#' {
                gen.insert((x as i32, y as i32, 0, 0), true);
            }
        }
    }
    println!("{:?}", gen);
    for i in 0..6 {
        gen = next(&gen);
    }
    println!("{:?}", gen.len());
}

fn next(gen: &HashMap<(i32, i32, i32, i32), bool>) -> HashMap<(i32, i32, i32, i32), bool> {
    let mut next: HashMap<(i32, i32, i32, i32), bool> = HashMap::new();
    let ((xmin, xmax), (ymin, ymax), (zmin, zmax), (wmin, wmax)) = limits(gen);
    for x in xmin..=xmax {
        for y in ymin..=ymax {
            for z in zmin..=zmax {
                for w in wmin..=wmax {
                    let p = (x, y, z, w);
                    let n = neighbors(p)
                        .iter()
                        .filter(|p| gen.contains_key(p))
                        .collect::<Vec<_>>()
                        .len();
                    if n == 3 {
                        next.insert(p, true);
                    }
                    if n == 2 {
                        if let Some(_) = gen.get(&p) {
                            next.insert(p, true);
                        }
                    }
                }
            }
        }
    }
    next
}

fn limits(
    gen: &HashMap<(i32, i32, i32, i32), bool>,
) -> ((i32, i32), (i32, i32), (i32, i32), (i32, i32)) {
    let mut xmin = i32::MAX;
    let mut xmax = i32::MIN;
    let mut ymin = i32::MAX;
    let mut ymax = i32::MIN;
    let mut zmin = i32::MAX;
    let mut zmax = i32::MIN;
    let mut wmin = i32::MAX;
    let mut wmax = i32::MIN;
    for (&k, _) in gen {
        let (x, y, z, w) = k;
        if x > xmax {
            xmax = x;
        }
        if x < xmin {
            xmin = x;
        }
        if y > ymax {
            ymax = y;
        }
        if y < ymin {
            ymin = y;
        }
        if z > zmax {
            zmax = z;
        }
        if z < zmin {
            zmin = z;
        }
        if w > wmax {
            wmax = w;
        }
        if w < wmin {
            wmin = w;
        }
    }
    (
        (xmin - 1, xmax + 1),
        (ymin - 1, ymax + 1),
        (zmin - 1, zmax + 1),
        (wmin - 1, wmax + 1),
    )
}

fn neighbors(p: (i32, i32, i32, i32)) -> Vec<(i32, i32, i32, i32)> {
    let (x, y, z, w) = p;
    let mut ns = Vec::new();
    for i in x - 1..=x + 1 {
        for j in y - 1..=y + 1 {
            for k in z - 1..=z + 1 {
                for l in w - 1..=w + 1 {
                    if (i, j, k, l) != p {
                        ns.push((i, j, k, l));
                    }
                }
            }
        }
    }
    // let ns: Vec<Vec<_>> = (x - 1..=x + 1)
    //     .map(|x| {
    //         (y - 1..=y + 1)
    //             .flat_map(|y| (z - 1..=z + 1).map(|z| (x, y, z)).collect())
    //             .collect()
    //     })
    //     .collect();
    ns
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
