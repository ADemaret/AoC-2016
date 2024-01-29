use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 06 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day06_input_demo1.txt");
    let input = include_str!("../assets/day06_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> String {
    // get stats
    let nbr_col = input.lines().next().unwrap().len();
    let mut data = Vec::new();
    for _ in 0..nbr_col {
        data.push(HashMap::new());
    }
    input.lines().for_each(|line| {
        line.chars().enumerate().for_each(|(i, c)| {
            if let Some(nbr) = &data[i].get(&c) {
                let val: usize = **nbr;
                data[i].insert(c, val + 1);
            } else {
                data[i].insert(c, 1);
            }
        });
    });
    // init code
    let mut code = Vec::new();
    code.resize(nbr_col, ' ');

    // get code
    for col in 0..nbr_col {
        let mut min = usize::MAX;
        for item in &data[col] {
            if *item.1 < min {
                min = *item.1;
                code[col] = *item.0;
            }
        }
    }
    code.iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day06_input_demo1.txt")),
            "advent"
        );
        assert_eq!(
            get_answer(include_str!("../assets/day06_input.txt")),
            "pljvorrk"
        );
    }
}
