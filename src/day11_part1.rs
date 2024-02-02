use crate::day11::data::Chemical;
use crate::day11::data::Equipment;
use itertools::Itertools;
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

    let input = include_str!("../assets/day11_input.txt");
    // let input = include_str!("../assets/day11_input_demo1.txt");
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

    let mut dejavu: DejavuType = Vec::new();
    let level = 0;
    dfs(&mut floors, current_floor, &mut dejavu, level)
}

type DejavuType = Vec<(Vec<Vec<Component>>, usize, isize, (Component, Component))>;

fn dfs(
    floors: &mut Vec<Vec<Component>>,
    current_floor: usize,
    dejavu: &mut DejavuType,
    level: usize,
) -> Option<usize> {
    // trouvé
    if is_won(floors) {
        debug!("WON !! at level {level}");
        return Some(0);
    }
    debug!("level {level}");

    // pause::pause();

    let mut v_in_elevator: Vec<(Component, Component)> = Vec::new();
    let mut floors_with_none = floors[current_floor].clone();
    floors_with_none.push(Component {
        chm: Chemical::None,
        equ: Equipment::None,
    });
    for (a, b) in (0..floors_with_none.len()).tuple_combinations() {
        v_in_elevator.push((floors_with_none[a], floors_with_none[b]));
    }

    let mut debug_string = "possible components in elevator are :".to_string();
    for in_elevator in &v_in_elevator {
        debug_string.push_str(format!("({},{})", in_elevator.0, in_elevator.1).as_str());
    }
    debug!("{}", debug_string);

    let mut min_dist = None;
    let new_level = level;

    'main_loop: for direction in [1, -1] {
        for in_elevator in &v_in_elevator {
            let mut floors_clone = floors.clone();
            let floors_clone2 = floors.clone();

            if is_allowed(floors, current_floor, direction, *in_elevator)
                && !dejavu.contains(&(floors_clone.clone(), current_floor, direction, *in_elevator))
            {
                // set dejavu
                dejavu.push((floors_clone.clone(), current_floor, direction, *in_elevator));

                // do it
                let new_floor = do_it(&mut floors_clone, current_floor, direction, *in_elevator);

                let new_dist = dfs(&mut floors_clone, new_floor, dejavu, new_level + 1);

                // reset dejavu
                let index = dejavu
                    .iter()
                    .position(|x| {
                        *x == (
                            floors_clone2.clone(),
                            current_floor,
                            direction,
                            *in_elevator,
                        )
                    })
                    .unwrap();
                dejavu.remove(index);

                // get min
                if new_dist.is_some() {
                    min_dist = Some(min_dist.unwrap_or(usize::MAX).min(new_dist.unwrap() + 1));
                }
            } else {
                debug!(
                    "{} {}, ({},{}) not allowed ",
                    current_floor, direction, in_elevator.0, in_elevator.1
                );
            }
        }
    }
    min_dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        // assert_eq!(
        //     get_answer(include_str!("../assets/day11_input_demo1.txt")),
        //     0
        // );
        assert_eq!(
            get_answer(include_str!("../assets/day11_input.txt")),
            Some(0)
        );
    }
}
