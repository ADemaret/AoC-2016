use std::{cmp::Reverse, time::Instant};

use itertools::Itertools;
use regex::Regex;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 04 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day04_input_demo1.txt");
    let input = include_str!("../assets/day04_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            if let Some(val) = is_north_pole(line) {
                val
            } else {
                0
            }
        })
        .sum()
}

fn is_north_pole(room: &str) -> Option<usize> {
    if let Ok(reg) = Regex::new(r"(?P<room>.*)(?P<sector>\d{3})\[(?P<checksum>\w{5})\]") {
        if let Some(caps) = reg.captures(room) {
            // get parts
            let room_chars = &caps["room"];
            let sector = &caps["sector"]
                .parse::<usize>()
                .expect("should have valid sector");
            let checksum = &caps["checksum"].chars().collect::<Vec<_>>();

            // get distinct chars
            let mut v_room_chars = room_chars
                .chars()
                .filter(|&x| x != '-')
                .sorted()
                .collect::<Vec<_>>();
            v_room_chars.dedup();
            let distinct_chars = v_room_chars.clone();

            // get frequence
            let mut freq = Vec::new();
            for c in distinct_chars {
                let nbr = room_chars
                    .chars()
                    .filter(|x| x == &c)
                    .collect::<Vec<_>>()
                    .len();
                freq.push((c, nbr));
            }
            freq.sort_by_key(|k| (Reverse(k.1), k.0));
            for i in 0..5 {
                if freq[i].0 != checksum[i] {
                    return None;
                }
            }

            // part2
            let decrypted = decrypt(room_chars, *sector);
            //println!("{decrypted}");
            if decrypted == "northpole object storage " {
                return Some(*sector);
            }
        } else {
            panic!();
        }
    }
    None
}

fn decrypt(encrypted: &str, sector: usize) -> String {
    encrypted
        .chars()
        .map(|c| {
            if c == '-' {
                ' '
            } else {
                let mut v = c as u8;
                v += (sector % 26) as u8;
                if v > b'z' {
                    v -= 26;
                }
                v as char
            }
        })
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert!(is_north_pole("aaaaa-bbb-z-y-x-123[abxyz]").is_none());
        assert!(is_north_pole("a-b-c-d-e-f-g-h-987[abcde]").is_none());
        assert!(is_north_pole("not-a-real-room-404[oarel]").is_none());
        assert!(is_north_pole("totally-real-room-200[decoy]").is_none());
        assert_eq!(decrypt("qzmt-zixmtkozy-ivhz", 343), "very encrypted name");
        assert_eq!(get_answer(include_str!("../assets/day04_input.txt")), 267);
    }
}
