use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 19 - Part 2 --");
    let now = Instant::now();

    // let input = 10;
    let input = 3_018_458;

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

#[derive(Debug)]
struct Elf {
    new_id: usize,
    gifts: usize,
}

fn get_answer(input: usize) -> usize {
    let mut elves = Vec::new();
    for i in 0..input {
        elves.push(Elf {
            new_id: i,
            gifts: 1,
        })
    }
    let mut id = 0;
    let mut remaining = input;
    loop {
        if elves[id].gifts > 0 {
            if let Some(giver_id) = next_gift(&mut elves, remaining, id) {
                // println!("Elf {} takes presents from elf {}", id, giver_id);

                elves[id].gifts += elves[giver_id].gifts; // elf takes all gifts
                elves[giver_id].gifts = 0;
                elves[giver_id].new_id = 0;
                for e in elves.iter_mut().skip(giver_id + 1) {
                    if e.new_id > 0 {
                        e.new_id -= 1;
                    }
                }
                remaining -= 1;
            } else {
                return id + 1;
            }
        }
        id += 1;
        if id == input {
            id = 0;
        }
    }
}

fn next_gift(elves: &mut [Elf], remaining: usize, elf_org_id: usize) -> Option<usize> {
    // how many elves in the circle ?
    if remaining % 1000 == 0 {
        println!("{remaining} remaining elves");
    }
    if remaining == 1 {
        return None;
    }
    // let mut correct_offset = offset % remaining;
    // if next_round {
    //     correct_offset = elves
    //         .iter()
    //         .enumerate()
    //         .position(|(i, x)| i == elf_org_id && x.gifts > 0)
    //         .unwrap();
    // }
    let giver_new_id =
        ((f32::floor(remaining as f32 / 2.0) as usize) + elves[elf_org_id].new_id) % remaining;
    let giver_org_id = elves
        .iter()
        .position(|x| x.new_id == giver_new_id && x.gifts > 0)
        .unwrap();
    // println!(
    //     "calcul : giver_id = rem/2 + offset = {remaining}/2 + {} = {} => id {}",
    //     offset, giver_new_id, giver_org_id
    // );
    Some(giver_org_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer(5), 2);
        assert_eq!(get_answer(10), 1);
        assert_eq!(get_answer(3018458), 1424135);
    }
}
