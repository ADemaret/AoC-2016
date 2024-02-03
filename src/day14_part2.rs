use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 14 - Part 2 --");
    let now = Instant::now();

    // let input = "abc";
    let input = "ihaygndm";
    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> usize {
    let mut index = 0;
    let mut valid_key = 0;
    let mut md5_cache: HashMap<usize, String> = HashMap::new();

    loop {
        // get md5
        let valid_index = next_valid_index(&mut md5_cache, input, index);
        // println!("valid key {} : {}", valid_key + 1, valid_index);
        valid_key += 1;
        if valid_key == 64 {
            return valid_index;
        }
        index = valid_index + 1;
    }
}

fn next_valid_index(md5_cache: &mut HashMap<usize, String>, input: &str, index: usize) -> usize {
    let mut valid_index = index;
    loop {
        let md5 = get_md5_string(md5_cache, input, valid_index);
        if let Some(triplet) = find_triplet(md5) {
            for i in 0..1000 {
                let next_md5 = get_md5_string(md5_cache, input, valid_index + i + 1);
                if contains_five_chars(next_md5, triplet) {
                    return valid_index;
                }
            }
        }
        valid_index += 1;
    }
}

fn get_md5_string(md5_cache: &mut HashMap<usize, String>, input: &str, index: usize) -> String {
    let mut md5_str;
    if let Some(md5) = md5_cache.get(&index) {
        md5_str = md5.to_string();
    } else {
        md5_str = format!("{input}{index}");
        for _ in 0..=2016 {
            md5_str = format!("{:?}", md5::compute(md5_str));
        }
        md5_cache.insert(index, md5_str.clone());
    }
    md5_str
}

fn find_triplet(str: String) -> Option<char> {
    for i in 0..(str.len() - 2) {
        if str.chars().nth(i) == str.chars().nth(i + 1)
            && str.chars().nth(i + 1) == str.chars().nth(i + 2)
        {
            return Some(str.chars().nth(i).unwrap());
        }
    }
    None
}

fn contains_five_chars(str: String, c: char) -> bool {
    for i in 0..(str.len() - 4) {
        if str.chars().nth(i).unwrap() == c
            && str.chars().nth(i + 1).unwrap() == c
            && str.chars().nth(i + 2).unwrap() == c
            && str.chars().nth(i + 3).unwrap() == c
            && str.chars().nth(i + 4).unwrap() == c
        {
            return true;
        }
    }
    false
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_md5_string() {
        assert_eq!(
            get_md5_string(&mut HashMap::new(), "abc", 0)
                .chars()
                .take(6)
                .collect::<String>(),
            "a107ff".to_string()
        );
    }

    #[test]
    fn test_contains_five_chars() {
        assert!(!contains_five_chars("111111111".to_string(), '0'));
        assert!(contains_five_chars("abcccccde".to_string(), 'c'));
        assert!(contains_five_chars("11111zz".to_string(), '1'));
        assert!(contains_five_chars("zz11111".to_string(), '1'));
    }

    #[test]
    fn test_find_triplet() {
        assert_eq!(find_triplet("abc".to_string()), None);
        assert_eq!(find_triplet("abbbc".to_string()), Some('b'));
        assert_eq!(find_triplet("abbbcddde".to_string()), Some('b'));
    }

    #[test]
    fn test_first_demo_triplet() {
        let md5 = get_md5_string(&mut HashMap::new(), "abc", 5);
        assert_eq!(find_triplet(md5), Some('2'));
        for i in (18 + 1)..(18 + 1000) {
            let next_md5 = get_md5_string(&mut HashMap::new(), "abc", i);
            assert!(!contains_five_chars(next_md5, '2'));
        }
    }

    #[test]
    fn test_second_demo_triplet() {
        let md5 = get_md5_string(&mut HashMap::new(), "abc", 10);
        assert_eq!(find_triplet(md5), Some('e'));
        let mut found = false;
        for i in (39 + 1)..(39 + 1000) {
            let next_md5 = get_md5_string(&mut HashMap::new(), "abc", i);
            if !contains_five_chars(next_md5, 'e') {
                found = true;
            }
        }
        assert!(found);
    }

    #[test]
    fn test_last_demo_triplets() {
        let md5 = get_md5_string(&mut HashMap::new(), "abc", 22551);
        assert_eq!(find_triplet(md5), Some('f'));
        let mut found = false;
        for i in (92 + 1)..(92 + 1000) {
            let next_md5 = get_md5_string(&mut HashMap::new(), "abc", i);
            if !contains_five_chars(next_md5, 'f') {
                found = true;
            }
        }
        assert!(found);
    }

    #[test]
    fn test_total() {
        assert_eq!(get_answer("abc"), 22551);
        assert_eq!(get_answer("ihaygndm"), 19968);
    }
}
