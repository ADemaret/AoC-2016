use std::{collections::VecDeque, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 13 - Part 1 --");
    let now = Instant::now();

    if let Some(sol) = get_answer(1352, Node { x: 31, y: 39 }) {
        // if let Some(sol) = get_answer(10, Node { x: 7, y: 4 }) {
        println!("La réponse est {sol}");
    } else {
        println!("Pas de réponse trouvée");
    }

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

#[derive(Debug, Clone, PartialEq)]
struct Node {
    x: usize,
    y: usize,
}

fn get_answer(input: usize, end_node: Node) -> Option<usize> {
    // print (to verify formula)
    // for y in 0..7 {
    //     for x in 0..10 {
    //         if is_open_space(input, x, y) {
    //             print!(".")
    //         } else {
    //             print!("#");
    //         }
    //     }
    //     println!();
    // }

    // breadth first search
    let mut dejavu = Vec::new();
    let mut queue = VecDeque::new();
    let start_node = Node { x: 1, y: 1 };

    // possibilities from start node
    let adjacent_nodes = get_adjacent_nodes(input, start_node.clone());
    for node in adjacent_nodes {
        queue.push_back((node, 1));
    }

    while let Some((current_node, step)) = queue.pop_front() {
        //println!("Step: {steps}, check: {}", queue.len());
        //print_state(&floors, current_floor);

        //dejavu.push((floors.clone(), current_floor, direction, in_elevator));
        // trouvé
        if current_node == end_node {
            //println!("WON !! in {step}");
            return Some(step);
        }

        if !dejavu.contains(&current_node) {
            dejavu.push(current_node.clone());

            let adjacent_nodes = get_adjacent_nodes(input, current_node.clone());
            for node in adjacent_nodes {
                if !dejavu.contains(&node) {
                    queue.push_back((node, step + 1));
                }
            }
        }
    }
    // not found
    None
}

fn get_adjacent_nodes(input: usize, current_node: Node) -> Vec<Node> {
    let mut adjacent_nodes = Vec::new();
    for direction in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let new_x = current_node.x as isize + direction.0;
        let new_y = current_node.y as isize + direction.1;
        if new_x >= 0 && new_y >= 0 && new_x <= 100 && new_y <= 100 {
            let new_node = Node {
                x: new_x as usize,
                y: new_y as usize,
            };
            if is_open_space(input, new_node.clone()) {
                adjacent_nodes.push(new_node);
            }
        }
    }
    adjacent_nodes
}

fn is_open_space(input: usize, node: Node) -> bool {
    let x = node.x;
    let y = node.y;
    let formula = (x * x + 3 * x + 2 * x * y + y + y * y) + input;
    let f_str = format!("{:b}", formula);
    let ones = f_str.chars().filter(|&x| x == '1').count();
    if ones % 2 == 0 {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer(10, Node { x: 7, y: 4 }), Some(11));
        assert_eq!(get_answer(1352, Node { x: 31, y: 39 }), Some(90));
    }
}
