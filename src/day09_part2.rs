use std::time::Instant;
use thousands::Separable;

use regex::Regex;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 09 - Part 2 --");
    let now = Instant::now();

    // let input = "(3x3)XYZ";
    // let input = "X(8x2)(3x3)ABCY";
    // let input = "(27x12)(20x12)(13x14)(7x10)(1x12)A";
    // let input = "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN";
    let input = include_str!("../assets/day09_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

#[derive(Debug, Default, Clone)]
struct FileItems {
    marker_chars: usize,
    marker_repeat: usize,
    str_len: usize,
}

fn get_answer(input: &str) -> usize {
    // println!("input = {}", input);

    // parse
    let mut v_file_items = get_file_items(input);

    // get size
    //let mut size = 0;
    let mut index = 0;
    let mut sub_vec = Vec::new();
    loop {
        if v_file_items.len() % 100 == 0 {
            println!(
                "index : {} / {}",
                index.separate_with_commas(),
                v_file_items.len().separate_with_commas()
            );
        }
        let first_element = v_file_items[index].clone();
        if first_element.marker_chars == 0 {
            // just a len a chars
            //size += first_element.str_len;
            index += 1;
        } else {
            let repeat = first_element.marker_repeat;
            let chars_len = first_element.marker_chars;
            let mut sub_len = 0;
            let mut sub_index = index + 1;
            // which elements will be multiplied
            while sub_len < chars_len {
                sub_len += v_file_items[sub_index].str_len;
                // if last element is not fully copied (too long)
                if sub_len > chars_len {
                    // ex : str_len 4, too-much = 1 => str_len 3 + str_len 1
                    let too_much = sub_len - chars_len;
                    let current_len = v_file_items[sub_index].str_len; // 4
                    let mut a_file_item = v_file_items[sub_index].clone();
                    a_file_item.str_len = current_len - too_much; // 3
                    v_file_items[sub_index] = a_file_item.clone(); // 3
                    sub_vec.push(a_file_item.clone()); // 3
                    v_file_items.insert(sub_index + 1, a_file_item.clone());
                    v_file_items[sub_index + 1].str_len = too_much; // 1
                } else {
                    sub_vec.push(v_file_items[sub_index].clone());
                }
                sub_index += 1;
            }

            if !sub_vec.is_empty() {
                sub_vec.reverse();
                for _ in 0..(repeat - 1) {
                    for el in &sub_vec {
                        v_file_items.insert(index + 1, el.clone());
                    }
                }
                v_file_items.remove(index);
            }
            sub_vec.clear();

            // group pure chars
            if v_file_items[0].marker_chars == 0 {
                while v_file_items.len() > 1 && v_file_items[1].marker_chars == 0 {
                    v_file_items[0].str_len += v_file_items[1].str_len;
                    v_file_items.remove(1);
                }
                index = 1;
            }
        }
        if index >= v_file_items.len() {
            break;
        }
    }
    v_file_items.iter().map(|x| x.str_len).sum()
}

fn get_file_items(input: &str) -> Vec<FileItems> {
    let mut ret_vec = Vec::new();
    let mut str = input.to_string();
    while !str.is_empty() && str.contains('(') {
        if let Ok(reg) =
            Regex::new(r"(?P<start>^[A-Z]*)\((?P<take>\d*)x(?P<repeat>\d*)\)(?P<end>.*)")
        {
            // characters outside of markers are replaced by their length
            if let Some(caps) = reg.captures(&str.clone()) {
                // println!("start  {:?}", &caps["start"]);
                let start_len = format!("{:?}", &caps["start"])
                    .to_string()
                    .trim_matches(|c| c == '\"')
                    .to_string()
                    .len();
                if start_len > 0 {
                    ret_vec.push(FileItems {
                        str_len: start_len,
                        ..Default::default()
                    });
                }
                // println!("take   {:?}", &caps["take"]);
                // println!("repeat {:?}", &caps["repeat"]);
                let str_len = format!("({:?}x{:?})", &caps["take"], &caps["repeat"])
                    .to_string()
                    .chars()
                    .filter(|&c| c != '\"')
                    .collect::<String>()
                    //.to_string()
                    .len();
                let marker_chars = format!("{:?}", &caps["take"])
                    .to_string()
                    .trim_matches(|c| c == '\"')
                    .to_string()
                    .parse::<usize>()
                    .unwrap();
                let marker_repeat = format!("{:?}", &caps["repeat"])
                    .to_string()
                    .trim_matches(|c| c == '\"')
                    .to_string()
                    .parse::<usize>()
                    .unwrap();
                ret_vec.push(FileItems {
                    marker_chars,
                    marker_repeat,
                    str_len,
                });
                // println!("end    {:?}", &caps["end"]);
                str = format!("{:?}", &&caps["end"])
                    .to_string()
                    .trim_matches(|c| c == '\"')
                    .to_string();
            }
        }
    }
    // don't forget to push final str
    let end_len = str
        .to_string()
        .trim_matches(|c| c == '\"')
        .to_string()
        .len();
    ret_vec.push(FileItems {
        str_len: end_len,
        ..Default::default()
    });

    // println!("=>{:#?}", ret_vec);
    ret_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer("(3x3)XYZ"), 9);
        assert_eq!(get_answer("X(8x2)(3x3)ABCY"), 20);
        assert_eq!(get_answer("(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241920);
        assert_eq!(
            get_answer("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"),
            445
        );
        // took 1:30 to get the answer !!
        assert_eq!(
            get_answer(include_str!("../assets/day09_input.txt")),
            11658395076
        );
    }
}
