use itertools::Itertools;
use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 03 - Part 2 --");
    let now = Instant::now();

    //let input = include_str!("../assets/day03_input_demo1.txt");
    let input = include_str!("../assets/day03_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> usize {
    let mut total = 0;
    for chunk in &input.lines().chunks(3) {
        let ch: Vec<_> = chunk.collect();
        let ch0 = ch[0]
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let ch1 = ch[1]
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let ch2 = ch[2]
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        if is_valid_triangle((ch0[0], ch1[0], ch2[0])) {
            total += 1;
        }
        if is_valid_triangle((ch0[1], ch1[1], ch2[1])) {
            total += 1;
        }
        if is_valid_triangle((ch0[2], ch1[2], ch2[2])) {
            total += 1;
        }
    }
    total
}

fn is_valid_triangle(side: (usize, usize, usize)) -> bool {
    side.0 < side.1 + side.2 && side.1 < side.0 + side.2 && side.2 < side.0 + side.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert!(!is_valid_triangle((5, 10, 25)));
        assert_eq!(get_answer(include_str!("../assets/day03_input.txt")), 1577);
    }
}
