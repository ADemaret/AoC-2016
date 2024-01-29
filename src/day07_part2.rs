use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 07 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day07_input_demo2.txt");
    let input = include_str!("../assets/day07_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> usize {
    input
        .lines()
        .map(|line| if supports_ssl(line) { 1 } else { 0 })
        .sum()
}

fn supports_ssl(line: &str) -> bool {
    let mut in_brackets = false;
    let mut aba_outside_brackets = Vec::new();
    let mut aba_in_brackets = Vec::new();
    let vline = line.chars().collect::<Vec<_>>();
    for i in 0..vline.len() - 2 {
        if vline[i] == '[' {
            in_brackets = true;
        } else if vline[i] == ']' {
            in_brackets = false;
        } else if vline[i] == vline[i + 2]
            && vline[i] != vline[i + 1]
            && !([']', '[']).contains(&vline[i + 1])
        {
            if in_brackets {
                aba_in_brackets.push(format!("{}{}{}", vline[i + 1], vline[i], vline[i + 1]));
            } else {
                aba_outside_brackets.push(format!("{}{}{}", vline[i], vline[i + 1], vline[i]));
            }
        }
    }
    for a in aba_in_brackets {
        if aba_outside_brackets.contains(&a) {
            //println!("{line} ok because of {}", &a);
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert!(supports_ssl("aba[bab]xyz"));
        assert!(!supports_ssl("xyx[xyx]xyx"));
        assert!(supports_ssl("aaa[kek]eke"));
        assert!(supports_ssl("zazbz[bzb]cdb"));
        assert_eq!(
            get_answer(include_str!("../assets/day07_input_demo2.txt")),
            3
        );
        assert_eq!(get_answer(include_str!("../assets/day07_input.txt")), 231);
    }
}
