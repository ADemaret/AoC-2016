use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 20 - Part 1 --");
    let now = Instant::now();

    //let input = include_str!("../assets/day20_input_demo1.txt");
    let input = include_str!("../assets/day20_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> usize {
    let mut raw_ranges = input
        .lines()
        .map(|line| {
            line.split_once('-')
                .map(|(a, b)| {
                    let min = a.parse::<usize>().unwrap();
                    let max = b.parse::<usize>().unwrap();
                    (min, max)
                })
                .unwrap()
        })
        .collect::<Vec<(usize, usize)>>();
    raw_ranges.sort_by_key(|x| x.0);
    // println!("{:?}", raw_ranges);
    // println!("ranges len : {}", raw_ranges.len());

    let mut ranges = Vec::new();
    let mut old_ranges = raw_ranges;
    loop {
        let mut skip_next = false;
        for i in 0..old_ranges.len() {
            if !skip_next {
                if i < old_ranges.len() - 1 {
                    if old_ranges[i].1 < old_ranges[i + 1].0 {
                        ranges.push(old_ranges[i]);
                    } else {
                        let max = old_ranges[i].1.max(old_ranges[i + 1].1);
                        ranges.push((old_ranges[i].0, max));
                        skip_next = true;
                    }
                } else {
                    ranges.push(old_ranges[i]);
                }
            } else {
                skip_next = false;
            }
        }
        if old_ranges.len() == ranges.len() {
            break;
        } else {
            old_ranges = ranges.clone();
            // println!("new len = {}", ranges.len());
            ranges.clear();
        }
    }

    ranges[0].1 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day20_input_demo1.txt")),
            3
        );
        assert_eq!(
            get_answer(include_str!("../assets/day20_input.txt")),
            17348574
        );
    }
}
