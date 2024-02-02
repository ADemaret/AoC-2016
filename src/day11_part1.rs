use crate::day11::data::Chemical;
use crate::day11::data::Equipment;
use itertools::Itertools;
use std::collections::VecDeque;
use std::time::Instant;
// personal functions
//use crate::utils::grid2d;
use crate::utils::pause;
//use crate::utils::math;
use crate::day11;
use crate::day11::data::Component;
use crate::day11::functions::*;
use crate::day11::parser::*;
use crate::utils::my_log;
use log::*;

pub fn main() {
    println!("-- Advent of Code - Day 11 - Part 1 --");
    let now = Instant::now();
    my_log::init_log();
    debug!("-- Advent of Code - Day 11 - Part 1 --");

    // let input = include_str!("../assets/day11_input_demo1.txt");
    // let mut floors = parse(input);
    // day11::demo::execute_demo(&mut floors);

    // let input = include_str!("../assets/day11_input_demo1.txt");
    let input = include_str!("../assets/day11_input.txt");
    if let Some(solution) = get_answer(input) {
        println!("La réponse est {}", solution);
    } else {
        println!("Pas de réponse trouvée");
    }

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> Option<usize> {
    // parse
    let mut floors = parse(input);

    // elevator starts a ground
    let current_floor = 0;
    //print_state(&floors, current_floor);

    bfs(&mut floors, current_floor)
}

// this is the magic trick to avoid checking too much states :
// it is not the exact microchips/generators at each floor that is important,
// it is the NUMBER of microchips/generators at each floor !
type DejavuType = Vec<(Vec<(usize, usize)>, usize)>; //, isize, (Component, Component))>;

fn get_elevator_possibilities(
    floors: &[Vec<Component>],
    current_floor: usize,
) -> Vec<(Component, Component)> {
    let mut v_in_elevator: Vec<(Component, Component)> = Vec::new();
    let mut floors_with_none = floors[current_floor].clone();
    floors_with_none.push(Component {
        chm: Chemical::None,
        equ: Equipment::None,
    });
    for (a, b) in (0..floors_with_none.len()).tuple_combinations() {
        v_in_elevator.push((floors_with_none[a], floors_with_none[b]));
    }
    v_in_elevator
}

fn bfs(floors: &mut Vec<Vec<Component>>, current_floor: usize) -> Option<usize> {
    let mut dejavu: DejavuType = Vec::new();
    let mut queue = VecDeque::new();
    let steps = 0;

    // possibilities from start node
    let v_in_elevator = get_elevator_possibilities(floors, current_floor);
    for direction in [1, -1] {
        for in_elevator in &v_in_elevator {
            queue.push_back((
                floors.clone(),
                current_floor,
                direction,
                *in_elevator,
                steps,
            ));
        }
    }

    while let Some((floors, current_floor, direction, in_elevator, steps)) = queue.pop_front() {
        //println!("Step: {steps}, check: {}", queue.len());
        //print_state(&floors, current_floor);

        //dejavu.push((floors.clone(), current_floor, direction, in_elevator));
        // trouvé
        if is_won(&floors) {
            debug!("WON !! in {steps}");
            return Some(steps);
        }

        let mut new_floors = floors.clone();
        if is_allowed(&new_floors, current_floor, direction, in_elevator) {
            // do it
            let new_floor = do_it(&mut new_floors, current_floor, direction, in_elevator);

            // set dejavu
            //dejavu.push((floors.clone(), current_floor, direction, in_elevator));
            let sum_mg = get_sum_mg(&new_floors);
            if !dejavu.contains(&(sum_mg.clone(), new_floor)) {
                dejavu.push((sum_mg, new_floor));

                // Check the following neighboring nodes for any that we've not visited yet.
                let v_in_elevator = get_elevator_possibilities(&new_floors, new_floor);
                for direction in [1, -1] {
                    for in_elevator in &v_in_elevator {
                        queue.push_back((
                            new_floors.clone(),
                            new_floor,
                            direction,
                            *in_elevator,
                            steps + 1,
                        ));
                    }
                }
            }
        }
    }
    // not found
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day11_input_demo1.txt")),
            Some(11)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day11_input.txt")),
            Some(37)
        );
    }
}
