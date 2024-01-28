use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 01 - Part 2 --");
    let now = Instant::now();

    let input = include_str!("../assets/day01_input.txt");

    if let Some(solution) = get_answer(input) {
        println!("La réponse est {}", solution);
    } else {
        println!("Pas de réponse trouvée");
    }

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> Option<usize> {
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut dir_index = 0; // north
    let mut blocks = (0, 0);
    let mut visited_blocks = Vec::new();
    let mut solution = None;
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
            for _step in 0..val {
                blocks.0 += dirs[dir_index].0;
                blocks.1 += dirs[dir_index].1;
                if solution.is_none() && visited_blocks.contains(&blocks) {
                    solution = Some(blocks);
                } else {
                    visited_blocks.push(blocks);
                }
            }
        });
    solution.map(|(x, y)| (isize::abs(x) + isize::abs(y)) as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer("R8, R4, R4, R8"), Some(4));
        assert_eq!(
            get_answer(include_str!("../assets/day01_input.txt")),
            Some(181)
        );
    }
}
