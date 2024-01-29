use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 07 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day07_input_demo1.txt");
    let input = include_str!("../assets/day07_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> usize {
    input
        .lines()
        .map(|line| if supports_tls(line) { 1 } else { 0 })
        .sum()
}

fn supports_tls(line: &str) -> bool {
    let mut in_brackets = false;
    let mut abba_outside_brackets = false;
    let mut abba_in_brackets = false;
    let vline = line.chars().collect::<Vec<_>>();
    for i in 0..vline.len() - 3 {
        if vline[i] == '[' {
            in_brackets = true;
        } else if vline[i] == ']' {
            in_brackets = false;
        } else if vline[i] == vline[i + 3]
            && vline[i + 1] == vline[i + 2]
            && vline[i] != vline[i + 1]
        {
            if in_brackets {
                abba_in_brackets = true;
            } else {
                abba_outside_brackets = true;
            }
        }
    }
    abba_outside_brackets && !abba_in_brackets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day07_input_demo1.txt")),
            2
        );
        assert_eq!(get_answer(include_str!("../assets/day07_input.txt")), 115);
    }
}
