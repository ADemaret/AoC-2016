use std::time::Instant;

// personal functions
use crate::utils::grid2d::Grid2D;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 18 - Part 1 and 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day18_input_demo1.txt");
    // println!("La réponse est {}", get_answer(input, 3));

    // let input = include_str!("../assets/day18_input_demo2.txt");
    // println!("La réponse est {}", get_answer(input, 10));

    let input = include_str!("../assets/day18_input.txt");
    println!("La réponse 1 est {}", get_answer(input, 40));
    println!("La réponse 2 est {}", get_answer(input, 400000));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str, rows: usize) -> usize {
    let mut grid = Grid2D::new_by_sizes(input.len(), rows, '.');
    grid.grid[0] = input.chars().collect::<Vec<_>>();
    for r in 1..rows {
        for c in 0..grid.max_c {
            let left;
            let center;
            let right;
            if c == 0 {
                left = '.';
                center = grid.grid[r - 1][c];
                right = grid.grid[r - 1][c + 1];
            } else if c == grid.max_c - 1 {
                left = grid.grid[r - 1][c - 1];
                center = grid.grid[r - 1][c];
                right = '.';
            } else {
                left = grid.grid[r - 1][c - 1];
                center = grid.grid[r - 1][c];
                right = grid.grid[r - 1][c + 1];
            }
            if (left == '^' && center == '^' && right == '.')
                || (left == '.' && center == '^' && right == '^')
                || (left == '^' && center == '.' && right == '.')
                || (left == '.' && center == '.' && right == '^')
            {
                grid.grid[r][c] = '^';
            }
        }
    }
    // grid.print();
    grid.count_occurences('.')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day18_input_demo2.txt"), 10),
            38
        );
        assert_eq!(
            get_answer(include_str!("../assets/day18_input.txt"), 40),
            1939
        );
        assert_eq!(
            get_answer(include_str!("../assets/day18_input.txt"), 400000),
            19999535
        );
    }
}
