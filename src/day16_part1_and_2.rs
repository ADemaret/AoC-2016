use std::time::Instant;

use itertools::Itertools;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 16 - Part 1 and 2 --");
    let now = Instant::now();

    //println!("La réponse est {}", get_answer("10000", 20));
    println!("La réponse 1 est {}", get_answer("01111001100111011", 272));
    println!(
        "La réponse 2 est {}",
        get_answer("01111001100111011", 35651584)
    );

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str, len: usize) -> String {
    let mut str = input.to_string();
    while str.len() < len {
        str = dragonize(str);
    }
    str = generate_checksum(str.as_str(), len);
    str
}

fn dragonize(str: String) -> String {
    let b = str
        .chars()
        .map(|x| match x {
            '1' => '0',
            '0' => '1',
            _ => unreachable!(),
        })
        .rev()
        .collect::<String>();
    let mut dragon = str.to_owned();
    dragon.push('0');
    dragon.push_str(b.as_str());
    dragon
}

fn generate_checksum(str: &str, len: usize) -> String {
    let mut str2 = str.chars().take(len).collect::<String>();
    loop {
        str2 = checksum(str2.as_str());
        if str2.len() % 2 == 1 {
            return str2;
        }
    }
}

fn checksum(str: &str) -> String {
    let checksum = str
        .chars()
        .tuples()
        .map(|(a, b)| if a == b { '1' } else { '0' })
        .collect::<String>();
    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum() {
        assert_eq!(generate_checksum("110010110100", 12), "100");
    }

    #[test]
    fn test_dragonize() {
        assert_eq!(dragonize("1".to_string()), "100");
        assert_eq!(dragonize("0".to_string()), "001");
        assert_eq!(dragonize("11111".to_string()), "11111000000");
        assert_eq!(
            dragonize("111100001010".to_string()),
            "1111000010100101011110000"
        );
    }

    #[test]
    fn test_total() {
        assert_eq!(get_answer("10000", 20), "01100");
        assert_eq!(get_answer("01111001100111011", 272), "11111000111110000");
        assert_eq!(
            get_answer("01111001100111011", 35651584),
            "10111100110110100"
        );
    }
}
