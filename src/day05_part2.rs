use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 05 - Part 2 --");
    let now = Instant::now();

    // let input = "abc";
    let input = "uqwqemis";

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> String {
    let mut code = [' '; 8];
    let mut start_value = 1;
    loop {
        let (ch, pos) = find_next_hash(input, &mut start_value).unwrap();
        if code[pos as usize] == ' ' {
            code[pos as usize] = ch;
            if !code.contains(&' ') {
                break;
            }
        }
        start_value += 1;
    }
    code.iter().collect()
}

fn find_next_hash(input: &str, val: &mut usize) -> Option<(char, u32)> {
    loop {
        let str = format!("{}{}", input, val);
        let digest = md5::compute(str);
        let digest_str = format!("{:x}", digest);
        if digest_str.get(0..5).unwrap() == "00000" {
            let pos_str = digest_str.chars().nth(5).unwrap();
            let pos = pos_str.to_digit(10);
            if pos.is_some() && pos.unwrap() < 8 {
                let ch = digest_str.chars().nth(6).unwrap();
                return Some((ch, pos.unwrap()));
            }
        }
        *val += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer("abc"), "05ace8e3");
        assert_eq!(get_answer("uqwqemis"), "694190cd");
    }
}
