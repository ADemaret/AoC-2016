use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 19 - Part 1 --");
    let now = Instant::now();

    // let input = 5;
    let input = 3018458;

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: usize) -> usize {
    let mut elves = vec![1; input];
    let mut i = 0;
    loop {
        if elves[i] > 0 {
            if let Some(giver) = next_gift(&elves, i) {
                // println!(
                //     "Elf {} takes {} presents from elf {}",
                //     i + 1,
                //     &elves[giver],
                //     giver + 1
                // );
                elves[i] += elves[giver]; // elf takes all gifts
                elves[giver] = 0;
            } else {
                return i + 1;
            }
        }
        i += 1;
        if i == input {
            i = 0;
        }
    }
}

fn next_gift(elves: &Vec<usize>, elf: usize) -> Option<usize> {
    let max = elves.len();
    let mut index = elf + 1;
    if index == max {
        index = 0;
    }
    loop {
        if elves[index] > 0 {
            return Some(index);
        } else {
            index += 1;
            if index == max {
                index = 0;
            } else if index == elf {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer(5), 3);
        //assert_eq!(get_answer(3018458), 0);
    }
}
