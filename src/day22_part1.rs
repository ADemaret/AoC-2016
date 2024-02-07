use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 22 - Part 1 --");
    let now = Instant::now();

    let input = include_str!("../assets/day22_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

#[derive(Debug)]
struct Node {
    // x: usize,
    // y: usize,
    // size: usize,
    used: usize,
    avail: usize,
    // p_use: usize,
}

fn get_answer(input: &str) -> usize {
    // parse
    let nodes = input
        .lines()
        .skip(2)
        .map(|line| {
            let values = line
                .split(['-', ' ', 'x', 'y', 'T', '%'])
                .filter_map(|x| x.parse::<usize>().ok())
                .collect::<Vec<_>>();
            Node {
                // x: values[0],
                // y: values[1],
                // size: values[2],
                used: values[3],
                avail: values[4],
                // p_use: values[5],
            }
        })
        .collect::<Vec<_>>();

    let mut count = 0;
    for i in 0..nodes.len() {
        for j in 0..nodes.len() {
            if i != j && nodes[i].used > 0 && nodes[j].avail >= nodes[i].used {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer(include_str!("../assets/day22_input.txt")), 1020);
    }
}
