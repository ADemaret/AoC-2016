use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 03 - Part 1 --");
    let now = Instant::now();

    //let input = include_str!("../assets/day03_input_demo1.txt");
    let input = include_str!("../assets/day03_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let sides = line
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            if sides.len() == 3 && is_valid_triangle((sides[0], sides[1], sides[2])) {
                1
            } else {
                0
            }
        })
        .sum()
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
        assert_eq!(get_answer(include_str!("../assets/day03_input.txt")), 862);
    }
}
