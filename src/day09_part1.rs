use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 09 - Part 1 --");
    let now = Instant::now();

    // let input = "ADVENT";
    // let input = "A(1x5)BC";
    // let input = "(3x3)XYZ";
    // let input = "A(2x2)BCD(2x2)EFG";
    // let input = "(6x1)(1x3)A";
    // let input = "X(8x2)(3x3)ABCY";
    let input = include_str!("../assets/day09_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> usize {
    // println!("input = {}", input);
    let v_input = input.chars().collect::<Vec<_>>();
    let mut v_result = Vec::new();
    let mut index = 0;
    while index < v_input.len() {
        if v_input[index] != '(' {
            v_result.push(v_input[index]);
            index += 1;
        } else {
            let mut i2 = index;
            let mut ch = v_input[i2];
            let mut instructions = String::new();
            while ch != ')' {
                instructions.push(v_input[i2]);
                i2 += 1;
                ch = v_input[i2];
            }
            // on a "instructions"
            let v_instr = instructions
                .split(['(', 'x', ')'])
                .filter_map(|v| v.parse::<usize>().ok())
                .collect::<Vec<_>>();
            let char_to_take = v_instr[0];
            let nbr_repeat = v_instr[1];
            // println!(
            //     "on doit prendre {} char et le répéter {} fois",
            //     v_instr[0], v_instr[1]
            // );
            index = i2 + 1;
            let str_to_take = v_input
                .iter()
                .skip(index)
                .take(char_to_take)
                .collect::<Vec<_>>();
            for _ in 0..nbr_repeat {
                for c in str_to_take.iter() {
                    v_result.push(**c);
                }
            }
            index += char_to_take;
        }
    }
    let final_str = v_result
        .iter()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();
    // println!("on obtient \"{:?}\"", final_str);
    final_str.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer("ADVENT"), 6);
        assert_eq!(get_answer("A(1x5)BC"), 7);
        assert_eq!(get_answer("(3x3)XYZ"), 9);
        assert_eq!(get_answer("A(2x2)BCD(2x2)EFG"), 11);
        assert_eq!(get_answer("(6x1)(1x3)A"), 6);
        assert_eq!(get_answer("X(8x2)(3x3)ABCY"), 18);
        assert_eq!(
            get_answer(include_str!("../assets/day09_input.txt")),
            120765
        );
    }
}
