use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 23 - Part 1 and 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day23_input_demo1.txt");
    let input = include_str!("../assets/day23_input.txt");

    println!("La réponse 1 est {}", get_answer(input, 7));
    println!("La réponse 2 est {}", get_answer(input, 12));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str, c_value: isize) -> isize {
    // parse
    let mut instructions: Vec<(&str, &str, &str)> = input
        .lines()
        .map(|line| {
            let chunks = line
                .split([' ', ','])
                .filter(|x| !x.is_empty())
                .collect::<Vec<&str>>();
            match chunks[0] {
                "inc" | "dec" | "tgl" => (chunks[0], chunks[1], ""),
                "cpy" | "jnz" => (chunks[0], chunks[1], chunks[2]),
                _ => unreachable!("should be a valid instruction"),
            }
        })
        .collect();

    let mut register = [0; 4];
    register[0] = c_value;
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
                // if target is not a register, skip instruction
                if str2.parse::<isize>().is_ok() {
                    // nothing
                } else {
                    let reg_index = get_register_index(str2);
                    if let Ok(val) = str1.parse::<isize>() {
                        register[reg_index] = val;
                    } else {
                        let reg_from = get_register_index(str1);
                        register[reg_index] = register[reg_from];
                    }
                }
                current += 1;
                // println!(" => {:?}", register)
            }
            "jnz" => {
                // jump if not zero
                let mut jump = false;
                let val1;
                let val2;

                if let Ok(v1) = str1.parse::<usize>() {
                    val1 = v1;
                } else {
                    val1 = register[get_register_index(str1)] as usize;
                }
                if let Ok(v2) = str2.parse::<isize>() {
                    val2 = v2;
                } else {
                    val2 = register[get_register_index(str2)] as isize;
                }
                if val1 != 0 && val2 != 0 {
                    current = (current as isize + val2) as usize;
                    jump = true;
                }
                if !jump {
                    current += 1;
                }
            }
            "tgl" => {
                let next;
                if let Ok(val) = str1.parse::<isize>() {
                    if val != 0 {
                        next = (current as isize + str1.parse::<isize>().unwrap()) as usize;
                    } else {
                        next = current;
                    }
                } else {
                    let reg_index = get_register_index(str1);
                    if register[reg_index] != 0 {
                        next = (current as isize + register[reg_index]) as usize;
                    } else {
                        next = current;
                    }
                }
                if next == 18 {
                    println!("coucou");
                }
                if next < instructions.len() {
                    if instructions[next].2.is_empty() {
                        // one-argument instructions
                        if instructions[next].0 == "inc" {
                            instructions[next].0 = "dec";
                        } else {
                            instructions[next].0 = "inc";
                        }
                    } else {
                        // two-argument instructions
                        if instructions[next].0 == "jnz" {
                            instructions[next].0 = "cpy";
                        } else {
                            instructions[next].0 = "jnz";
                        }
                    }
                }
                current += 1;
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
            get_answer(include_str!("../assets/day23_input_demo1.txt"), 7),
            3
        );
        assert_eq!(
            get_answer(include_str!("../assets/day23_input.txt"), 7),
            12860
        );
        assert_eq!(
            get_answer(include_str!("../assets/day23_input.txt"), 12),
            479009420
        );
    }
}
