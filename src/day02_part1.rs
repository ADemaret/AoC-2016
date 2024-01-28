use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 02 - Part 1 --");
    let now = Instant::now();

    //let input = include_str!("../assets/day02_input_demo1.txt");
    let input = include_str!("../assets/day02_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> String {
    let keypad = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];
    let mut l: usize = 1;
    let mut c: usize = 1;
    let solution = input
        .lines()
        .map(|line| {
            for ch in line.chars() {
                match ch {
                    'U' => {
                        l = l.saturating_sub(1);
                    }
                    'D' => {
                        if l < 2 {
                            l += 1;
                        }
                    }
                    'L' => {
                        c = c.saturating_sub(1);
                    }
                    'R' => {
                        if c < 2 {
                            c += 1;
                        }
                    }
                    _ => unreachable!("should be a valid direction"),
                }
            }
            keypad[l][c]
        })
        .collect::<String>();
    solution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day02_input_demo1.txt")),
            "1985"
        );
        assert_eq!(
            get_answer(include_str!("../assets/day02_input.txt")),
            "52981"
        );
    }
}
