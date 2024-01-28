use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 01 - Part 1 --");
    let now = Instant::now();

    //let input = include_str!("../assets/day01_input_demo1.txt");
    let input = include_str!("../assets/day01_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> usize {
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut dir_index = 0; // north
    let mut blocks = (0, 0);
    input
        .split([' ', ','])
        .filter(|instr| !instr.is_empty())
        .for_each(|instr| {
            match instr.chars().take(1).collect::<Vec<char>>()[0] {
                'L' => {
                    dir_index = (dir_index + 3) % 4;
                }
                'R' => {
                    dir_index = (dir_index + 1) % 4;
                }
                _ => unreachable!("should be L or R"),
            }
            let val_str = instr.chars().skip(1).collect::<String>();
            let val = val_str.parse::<isize>().unwrap();
            blocks.0 += dirs[dir_index].0 * val;
            blocks.1 += dirs[dir_index].1 * val;
        });
    (isize::abs(blocks.0) + isize::abs(blocks.1)) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer("R2, L3"), 5);
        assert_eq!(get_answer("R2, R2, R2"), 2);
        assert_eq!(get_answer("R5, L5, R5, R3"), 12);
        assert_eq!(get_answer(include_str!("../assets/day01_input.txt")), 299);
    }
}
