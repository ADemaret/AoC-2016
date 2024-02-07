use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 21 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day21_input_demo1.txt");
    // println!("La réponse est {}", get_answer(input, "abcde"));

    let input = include_str!("../assets/day21_input.txt");
    println!("La réponse est {}", get_answer(input, "abcdefgh"));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str, start: &str) -> String {
    //parse
    let instructions = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut scrambled = start.chars().collect::<Vec<_>>();
    for instr in instructions {
        let buff = scrambled.clone();
        match instr[0] {
            "swap" => match instr[1] {
                "position" => {
                    let i1 = instr[2].parse::<usize>().unwrap();
                    let i2 = instr[5].parse::<usize>().unwrap();
                    scrambled[i1] = buff[i2];
                    scrambled[i2] = buff[i1];
                }
                "letter" => {
                    let ch1 = instr[2].chars().next().unwrap();
                    let ch2 = instr[5].chars().next().unwrap();
                    let pos1 = scrambled.iter().position(|&x| x == ch1).unwrap();
                    let pos2 = scrambled.iter().position(|&x| x == ch2).unwrap();
                    scrambled[pos1] = ch2;
                    scrambled[pos2] = ch1;
                }
                _ => unreachable!("Swap option should be valid"),
            },
            "rotate" => match instr[1] {
                "left" => {
                    let offset = instr[2].parse::<isize>().unwrap() as usize;
                    scrambled = rotate_left(buff, offset);
                }
                "right" => {
                    let offset = instr[2].parse::<isize>().unwrap();
                    scrambled = rotate_right(buff, offset as usize);
                }
                "based" => {
                    let ch1 = instr[6].chars().next().unwrap();
                    let pos1 = scrambled.iter().position(|&x| x == ch1).unwrap();
                    let mut offset = pos1 + 1;
                    if pos1 >= 4 {
                        offset += 1;
                    }
                    scrambled = rotate_right(buff, offset);
                }
                _ => unreachable!("Rotate option should be valid"),
            },
            "reverse" => {
                let i1 = instr[2].parse::<usize>().unwrap();
                let i2 = instr[4].parse::<usize>().unwrap();
                for i in 0..=((i2 as isize - i1 as isize) as usize) {
                    scrambled[i1 + i] = buff[i2 - i];
                }
            }
            "move" => {
                let i1 = instr[2].parse::<usize>().unwrap();
                let i2 = instr[5].parse::<usize>().unwrap();
                let ch = scrambled.remove(i1);
                scrambled.insert(i2, ch);
            }
            _ => unreachable!("Instructions should be defined"),
        }
    }

    scrambled.iter().collect::<String>()
}

fn rotate_left(buff: Vec<char>, offset: usize) -> Vec<char> {
    let len = buff.len();
    let offset = offset % len;
    let mut scrambled = buff.clone();
    let sep = (len as isize - offset as isize) as usize;
    scrambled[0..sep].copy_from_slice(&buff[offset..len]);
    scrambled[sep..len].copy_from_slice(&buff[0..offset]);
    scrambled
}

fn rotate_right(buff: Vec<char>, offset: usize) -> Vec<char> {
    let len = buff.len();
    let offset = offset % len;
    let mut scrambled = buff.clone();
    let sep = len - offset;
    scrambled[0..offset].copy_from_slice(&buff[sep..len]);
    scrambled[offset..len].copy_from_slice(&buff[0..sep]);
    scrambled
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instructions() {
        assert_eq!(
            get_answer("swap position 4 with position 0", "01234"),
            "41230"
        );
        assert_eq!(
            get_answer("swap position 1 with position 2", "01234"),
            "02134"
        );
        assert_eq!(get_answer("swap letter d with letter b", "abcd"), "adcb");
        assert_eq!(get_answer("rotate left 1 step", "0123"), "1230");
        assert_eq!(get_answer("rotate left 2 step", "0123"), "2301");
        assert_eq!(get_answer("rotate right 1 step", "0123"), "3012");
        assert_eq!(get_answer("rotate right 2 step", "0123"), "2301");
        assert_eq!(
            get_answer("move position 1 to position 4", "01234"),
            "02341"
        );
        assert_eq!(
            get_answer("move position 3 to position 0", "01234"),
            "30124"
        );
        assert_eq!(
            get_answer("rotate based on position of letter b", "abdec"),
            "ecabd"
        );
        assert_eq!(
            get_answer("rotate based on position of letter d", "ecabd"),
            "decab"
        );
    }

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day21_input_demo1.txt"), "abcde"),
            "decab".to_string()
        );
        assert_eq!(
            get_answer(include_str!("../assets/day21_input.txt"), "abcdefgh"),
            "dgfaehcb".to_string()
        );
    }
}
