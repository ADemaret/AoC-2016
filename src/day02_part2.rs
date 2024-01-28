use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 02 - Part 2 --");
    let now = Instant::now();

    //let input = include_str!("../assets/day02_input_demo1.txt");
    let input = include_str!("../assets/day02_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> String {
    let keypad = [
        [' ', ' ', '1', ' ', ' '],
        [' ', '2', '3', '4', ' '],
        ['5', '6', '7', '8', '9'],
        [' ', 'A', 'B', 'C', ' '],
        [' ', ' ', 'D', ' ', ' '],
    ];
    let mut l: usize = 2;
    let mut c: usize = 0;
    let solution = input
        .lines()
        .map(|line| {
            for ch in line.chars() {
                match ch {
                    'U' => {
                        if l > 0 && keypad[l - 1][c] != ' ' {
                            l -= 1;
                        }
                    }
                    'D' => {
                        if l < 4 && keypad[l + 1][c] != ' ' {
                            l += 1;
                        }
                    }
                    'L' => {
                        if c > 0 && keypad[l][c - 1] != ' ' {
                            c -= 1;
                        }
                    }
                    'R' => {
                        if c < 4 && keypad[l][c + 1] != ' ' {
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
            "5DB3"
        );
        assert_eq!(
            get_answer(include_str!("../assets/day02_input.txt")),
            "74CD2"
        );
    }
}
