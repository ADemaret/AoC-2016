use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 10 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day10_input_demo1.txt");
    let input = include_str!("../assets/day10_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

#[derive(Debug, Default, Clone, Copy, PartialEq)]
enum Destination {
    #[default]
    None,
    Output,
    Bot,
}

#[derive(Debug, Default)]
struct Robot {
    low: Option<usize>,
    high: Option<usize>,
    give_low: Destination,
    give_low_nr: usize,
    give_high: Destination,
    give_high_nr: usize,
}

fn get_answer(input: &str) -> usize {
    let mut robots = HashMap::new();
    let mut outputs: HashMap<usize, Option<usize>> = HashMap::new();
    // parse
    // first create bots and give original chips
    for line in input.lines() {
        let (what, end) = line.split_once(' ').unwrap();
        match what {
            "value" => {
                let (c, b) = end.split_once(" goes to bot ").unwrap();
                let chip = c.parse::<usize>().unwrap();
                let bot = b.parse::<usize>().unwrap();
                if give_chip(&mut robots, bot, chip, None) {
                    return bot;
                }
            }
            "bot" => {
                let parts = end.split_whitespace().collect::<Vec<&str>>(); // always 11 "parts"
                let bot_org = parts[0].parse::<usize>().unwrap();
                let bot_low = if parts[4] == "bot" {
                    Destination::Bot
                } else if parts[4] == "output" {
                    Destination::Output
                } else {
                    Destination::None
                };
                let bot_low_nr = parts[5].parse::<usize>().unwrap();
                // part2
                if parts[4] == "output" {
                    outputs.insert(bot_low_nr, None);
                }
                let bot_high = if parts[9] == "bot" {
                    Destination::Bot
                } else if parts[9] == "output" {
                    Destination::Output
                } else {
                    Destination::None
                };
                let bot_high_nr = parts[10].parse::<usize>().unwrap();
                // part2
                if parts[9] == "output" {
                    outputs.insert(bot_high_nr, None);
                }
                update_robots(
                    &mut robots,
                    bot_org,
                    bot_low,
                    bot_low_nr,
                    bot_high,
                    bot_high_nr,
                );
            }
            _ => unreachable!(),
        }
    }
    // then propagate
    loop {
        let mut starting_id = None;
        for id in robots.keys() {
            if is_ready(&robots, *id) {
                starting_id = Some(*id);
            }
        }
        if starting_id.is_none() {
            break;
        }
        if let Some(solution) = propagate(&mut robots, &mut outputs, starting_id.unwrap()) {
            return solution;
        }
    }
    //println!("{:#?}", &robots);
    //println!("{:#?}", &outputs);
    outputs.get(&0).unwrap().unwrap()
        * outputs.get(&1).unwrap().unwrap()
        * outputs.get(&2).unwrap().unwrap()
}

fn give_chip(
    robots: &mut HashMap<usize, Robot>,
    bot: usize,
    chip: usize,
    bot_from: Option<usize>,
) -> bool {
    // println!("le robot {} reçoit {} de {:?}",bot,chip,bot_from);
    // new robot
    // only one value = higher than none
    if robots.get(&bot).is_none() {
        robots.insert(
            bot,
            Robot {
                high: Some(chip),
                ..Default::default()
            },
        );
    } else {
        // existing robot
        let current = robots.get_mut(&bot).unwrap();
        // bot can't get third chip
        assert!(current.low.is_none() || current.high.is_none());

        if current.high.is_some() {
            let old_chip = current.high.unwrap();
            current.low = Some(chip.min(old_chip));
            current.high = Some(chip.max(old_chip));
        } else if current.low.is_some() {
            let old_chip = current.low.unwrap();
            current.low = Some(chip.min(old_chip));
            current.high = Some(chip.max(old_chip));
        } else {
            current.high = Some(chip);
        }
        // if current.low == Some(17) && current.high == Some(61) {
        //     return true;
        // }
    }
    if bot_from.is_some() {
        let giver = robots.get_mut(&bot_from.unwrap()).unwrap();
        if giver.low == Some(chip) {
            giver.low = None;
        } else if giver.high == Some(chip) {
            giver.high = None;
        }
    }
    false
}

fn destroy_chip(
    robots: &mut HashMap<usize, Robot>,
    outputs: &mut HashMap<usize, Option<usize>>,
    chip: usize,
    bot_from: usize,
    output_nr: usize,
) {
    // println!("le robot {} detruit {}",bot_from,chip);

    let giver = robots.get_mut(&bot_from).unwrap();
    if giver.low == Some(chip) {
        outputs.insert(giver.give_low_nr, Some(chip));
        giver.low = None;
    } else if giver.high == Some(chip) {
        outputs.insert(giver.give_high_nr, Some(chip));
        giver.high = None;
    }
}

fn update_robots(
    robots: &mut HashMap<usize, Robot>,
    bot_org: usize,
    give_low: Destination,
    give_low_nr: usize,
    give_high: Destination,
    give_high_nr: usize,
) {
    // new robot
    if robots.get(&bot_org).is_none() {
        robots.insert(
            bot_org,
            Robot {
                give_low,
                give_low_nr,
                give_high,
                give_high_nr,
                ..Default::default()
            },
        );
    } else {
        let current = robots.get_mut(&bot_org).unwrap();
        current.give_low = give_low;
        current.give_low_nr = give_low_nr;

        current.give_high = give_high;
        current.give_high_nr = give_high_nr;
    }

    // create destination bots if needed
    if give_low == Destination::Bot && robots.get_mut(&give_low_nr).is_none() {
        robots.insert(
            give_low_nr,
            Robot {
                ..Default::default()
            },
        );
    }
    if give_high == Destination::Bot && robots.get_mut(&give_high_nr).is_none() {
        robots.insert(
            give_high_nr,
            Robot {
                ..Default::default()
            },
        );
    }
}

fn is_ready(robots: &HashMap<usize, Robot>, bot_id: usize) -> bool {
    // bot has 2 chips
    // it has 2 destinations
    let current = robots.get(&bot_id).unwrap();
    if current.low.is_some()
        && current.high.is_some()
        && current.give_low != Destination::None
        && current.give_high != Destination::None
    {
        return true;
    }
    false
}

fn propagate(
    robots: &mut HashMap<usize, Robot>,
    outputs: &mut HashMap<usize, Option<usize>>,
    bot_id: usize,
) -> Option<usize> {
    //assert!(is_ready(robots,bot_id));

    let current = robots.get(&bot_id).unwrap();
    let receiver1 = current.give_low_nr;
    let receiver2 = current.give_high_nr;
    let gchip_low = current.give_low;
    let gchip_high = current.give_high;
    let chip_low = current.low.unwrap();
    let chip_high = current.high.unwrap();

    if gchip_low == Destination::Bot {
        // println!("le robot {} donne {} à {}",bot_id,chip_low, receiver1);
        if give_chip(robots, receiver1, chip_low, Some(bot_id)) {
            return Some(receiver1);
        }
    } else if gchip_low == Destination::Output {
        destroy_chip(robots, outputs, chip_low, bot_id, receiver1);
    }
    if gchip_high == Destination::Bot {
        // println!("le robot {} donne {} à {}",bot_id,chip_high, receiver2);
        if give_chip(robots, receiver2, chip_high, Some(bot_id)) {
            return Some(receiver2);
        }
    } else if gchip_high == Destination::Output {
        destroy_chip(robots, outputs, chip_high, bot_id, receiver2);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day10_input_demo1.txt")),
            30
        );
        assert_eq!(get_answer(include_str!("../assets/day10_input.txt")), 4042);
    }
}
