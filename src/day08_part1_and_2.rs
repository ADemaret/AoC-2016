use std::time::Instant;

// personal functions
use crate::utils::grid2d::Grid2D;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 08 - Part 1 and 2 --");
    let now = Instant::now();

    //let input = include_str!("../assets/day08_input_demo1.txt");
    let input = include_str!("../assets/day08_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> usize {
    //let mut screen = Grid2D::new_by_sizes(7, 3, '.');
    let mut screen = Grid2D::new_by_sizes(50, 6, '.');
    // parse
    input.lines().for_each(|line| {
        let v_val = line
            .split([' ', '=', 'x'])
            .filter_map(|str| str.parse::<usize>().ok())
            .collect::<Vec<_>>();
        let v_str = line.split_whitespace().collect::<Vec<_>>();
        if v_str[0] == "rect" {
            screen.set_rect(v_val[0], v_val[1], '#');
        } else if v_str[1] == "row" {
            screen.shift_row_right(v_val[0], v_val[1]);
        } else if v_str[1] == "column" {
            screen.shift_col_down(v_val[0], v_val[1]);
        } else {
            panic!();
        }
    });
    screen.print(); // part2
    screen.count_occurences('#')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer(include_str!("../assets/day08_input.txt")), 121);
    }
}
