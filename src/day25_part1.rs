use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 25 - Part 1 and 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day25_input_demo1.txt");
    let input = include_str!("../assets/day25_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> isize {
    // parse
    let instructions: Vec<(&str, &str, &str)> = input
        .lines()
        .map(|line| {
            let chunks = line
                .split([' ', ','])
                .filter(|x| !x.is_empty())
                .collect::<Vec<&str>>();
            match chunks[0] {
                "inc" | "dec" | "out" => (chunks[0], chunks[1], ""),
                "cpy" | "jnz" => (chunks[0], chunks[1], chunks[2]),
                _ => unreachable!("should be a valid instruction"),
            }
        })
        .collect();

    let mut solution = 0;
    let mut clock = String::new();
    while !clock.contains("010101010101010") && solution < 160 {
        solution += 1;
        clock = follow_instructions(&instructions, solution);
    }
    // println!("{solution} - {clock}");
    solution
}

fn get_register_index(register: &str) -> usize {
    (register.chars().next().unwrap() as u8 - b'a') as usize
}

fn follow_instructions(instructions: &Vec<(&str, &str, &str)>, register_a: isize) -> String {
    let mut clock = String::new();
    let mut counter = 0;
    let mut current = 0;
    let mut register = [0; 4];
    register[0] = register_a;
    while current < instructions.len() && counter < 100_000 {
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
            "out" => {
                let reg_index = get_register_index(str1);
                let c = format!("{}", register[reg_index]);
                clock.push(c.chars().next().unwrap());
                current += 1;
            }
            _ => unreachable!(),
        }
        counter += 1;
    }
    clock
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer(include_str!("../assets/day25_input.txt")), 158);
    }
}
