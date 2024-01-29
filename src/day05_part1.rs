use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 05 - Part 1 --");
    let now = Instant::now();

    // let input = "abc";
    let input = "uqwqemis";

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> String {
    let mut code = String::new();
    let mut start_value = 1;
    for _ in 0..8 {
        code.push(find_next_hash(input, &mut start_value).unwrap());
        start_value += 1;
    }
    code
}

fn find_next_hash(input: &str, val: &mut usize) -> Option<char> {
    loop {
        let str = format!("{}{}", input, val);
        let digest = md5::compute(str);
        let digest_str = format!("{:x}", digest);
        if digest_str.get(0..5).unwrap() == "00000" {
            return Some(digest_str.chars().nth(5).unwrap());
        }
        *val += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer("abc"), "18f47a30");
        assert_eq!(get_answer("uqwqemis"), "1a3099aa");
    }
}
