use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 12 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day12_input_demo1.txt");
    let input = include_str!("../assets/day12_input.txt");

    println!("La réponse 1 est {}", get_answer(input, 0));
    println!("La réponse 2 est {}", get_answer(input, 1));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str, c_value: isize) -> isize {
    // parse
    let instructions: Vec<(&str, &str, &str)> = input
        .lines()
        .map(|line| {
            let chunks = line
                .split([' ', ','])
                .filter(|x| !x.is_empty())
                .collect::<Vec<&str>>();
            match chunks[0] {
                "inc" | "dec" => (chunks[0], chunks[1], ""),
                "cpy" | "jnz" => (chunks[0], chunks[1], chunks[2]),
                _ => unreachable!("should be a valid instruction"),
            }
        })
        .collect();

    let mut register = [0; 4];
    register[2] = c_value;
    let mut current = 0;
    while current < instructions.len() {
        // println!("line {} : {:?}", current, instructions[current]);
        let (instr, str1, str2) = instructions[current];
        // print!("instr {} : {:?}", current, instructions[current]);
        match instr {
            "inc" => {
                let reg_index = get_register_index(str1);
                register[reg_index] += 1;
                current += 1;
                // println!(" => {:?}", register)
            }
            "dec" => {
                let reg_index = get_register_index(str1);
                register[reg_index] -= 1;
                current += 1;
                // println!(" => {:?}", register)
            }
            "cpy" => {
                // copy
                let reg_index = get_register_index(str2);
                if let Ok(val) = str1.parse::<isize>() {
                    register[reg_index] = val;
                } else {
                    let reg_from = get_register_index(str1);
                    register[reg_index] = register[reg_from];
                }
                current += 1;
                // println!(" => {:?}", register)
            }
            "jnz" => {
                // jump if not zero
                let mut jump = false;
                if let Ok(val) = str1.parse::<isize>() {
                    if val != 0 {
                        current = (current as isize + str2.parse::<isize>().unwrap()) as usize;
                        jump = true;
                    }
                } else {
                    let reg_index = get_register_index(str1);
                    if register[reg_index] != 0 {
                        current = (current as isize + str2.parse::<isize>().unwrap()) as usize;
                        jump = true;
                    }
                }
                if !jump {
                    current += 1;
                }
            }
            _ => unreachable!(),
        }
    }
    register[0]
}

fn get_register_index(register: &str) -> usize {
    (register.chars().next().unwrap() as u8 - b'a') as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day12_input_demo1.txt"), 0),
            42
        );
        assert_eq!(
            get_answer(include_str!("../assets/day12_input.txt"), 0),
            318020
        );
        assert_eq!(
            get_answer(include_str!("../assets/day12_input.txt"), 1),
            9227674
        );
    }
}
