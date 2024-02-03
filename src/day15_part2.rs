use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 15 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day15_input_demo1.txt");
    let input = include_str!("../assets/day15_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//
#[derive(Debug)]
struct Disk {
    id: usize,
    pos: usize,
    //dt: usize,
    dt_pos: usize,
}

fn get_answer(input: &str) -> usize {
    // parse
    let mut data = input
        .lines()
        .map(|line| {
            let parts = line
                .split([' ', '#', '=', ',', '.'])
                .filter_map(|x| x.parse::<usize>().ok())
                .collect::<Vec<_>>();
            Disk {
                id: parts[0],
                pos: parts[1],
                //dt: parts[2],
                dt_pos: parts[3],
            }
        })
        .collect::<Vec<_>>();

    data.push(Disk {
        id: 7,
        pos: 11,
        dt_pos: 0,
    });

    // for d in &data {
    //     println!("{:?}", d);
    //     for t in 0..7 {
    //         print!("{} ", (d.dt_pos + (t + d.id)) % d.pos);
    //     }
    //     println!();
    // }

    let mut index = 0;
    loop {
        let mut all_zero = true;
        for d in &data {
            if (d.dt_pos + (index + d.id)) % d.pos != 0 {
                all_zero = false;
            }
        }
        if all_zero {
            return index;
        }
        index += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day15_input.txt")),
            3208099
        );
    }
}
