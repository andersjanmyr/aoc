use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt");
    let data = input.unwrap();
    let foods: Vec<_> = parse_input(&data).collect();

    // Find out relationships between allergens and ingredients that may contain them:
    let mut atoi: HashMap<&str, HashSet<&str>> = HashMap::new();
    for food in &foods {
        let ingredient_set: HashSet<_> = food.ingredients.iter().map(|&s| s).collect();
        for allergen in &food.allergens {
            let curr_ingredients = atoi
                .entry(allergen)
                .or_insert_with(|| ingredient_set.clone());
            *curr_ingredients = curr_ingredients
                .intersection(&ingredient_set)
                .cloned()
                .collect();
        }
    }
    println!("{:?}", foods);
    println!("{:?}", atoi);

    // Squash that into a big set of "possibly-contains-allergen" ingredients
    let risky_ingredients: HashSet<&str> = atoi.values().flat_map(|s| s).cloned().collect();
    println!("Risky: {:?}", risky_ingredients);
    let ok_count = foods
        .iter()
        .flat_map(|f| f.ingredients.iter())
        .filter(|&i| !risky_ingredients.contains(i))
        .count();
    println!("Star 1: {}", ok_count);

    let mut di = Vec::new();
    while let Some((a, i)) = atoi.iter().find(|(_, is)| is.len() == 1) {
        let ing = *i.iter().next().unwrap();
        di.push((ing, a.to_string()));
        for (_, is) in &mut atoi {
            is.remove(ing);
        }
    }
    di.sort_by(|a, b| a.1.cmp(&b.1));
    println!(
        "Star 2: {:?}",
        di.iter().map(|(a, _)| *a).collect::<Vec<&str>>().join(",")
    );
}

#[derive(Debug, Clone)]
struct Food<'a> {
    ingredients: Vec<&'a str>,
    allergens: Vec<&'a str>,
}

fn parse_input<'a>(s: &'a str) -> impl Iterator<Item = Food<'a>> + 'a {
    s.lines().filter_map(|line| {
        let caps = Regex::new(r#"([a-z ]+) \(contains ([a-z, ]+)\)"#)
            .unwrap()
            .captures(line.trim())?;
        let ingredients = caps.get(1)?.as_str().split(" ").collect();
        let allergens = caps.get(2)?.as_str().split(", ").collect();
        Some(Food {
            ingredients,
            allergens,
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
