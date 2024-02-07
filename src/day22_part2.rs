use std::{collections::HashMap, time::Instant};

use crate::utils::grid2d::Grid2D;

// personal functions
//use crate::utils::grid2d;
use crate::utils::pause;
//use crate::utils::math;

const SHOW_STEPS: bool = false;

pub fn main() {
    println!("-- Advent of Code - Day 22 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day22_input_demo2.txt");
    let input = include_str!("../assets/day22_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

#[derive(Debug, Clone)]
struct Node {
    // size: usize,
    used: usize,
    avail: usize,
    // p_use: usize,
}

fn get_answer(input: &str) -> usize {
    // parse
    let (mut nodes, max_x, max_y) = parse(input);

    let start_node = (0, 0);
    let mut end_node = (0, max_x - 1);

    // NB : c = y / l = x
    let mut grid = Grid2D::new_by_sizes(max_x, max_y, '.');
    // print_nodes(&nodes, &mut grid, start_node, end_node);

    let mut moves = 0;
    // move the empty cell to left border
    for x in (0..4).rev() {
        move_data(&vec![(25, x), (25, x + 1)], &mut nodes);
        moves += 1;
        if SHOW_STEPS {
            print_nodes(&nodes, &mut grid, start_node, end_node);
            println!("empty goes from (25,{}) to (25,{})", x + 1, x);
            pause::pause();
        }
    }
    // move the empty cell up the "wall"
    for y in (15..25).rev() {
        move_data(&vec![(y, 0), (y + 1, 0)], &mut nodes);
        moves += 1;
        if SHOW_STEPS {
            print_nodes(&nodes, &mut grid, start_node, end_node);
            println!("empty goes from ({},0) to ({},0)", y + 1, y);
            pause::pause();
        }
    }
    // move the empty cell to right border
    for x in 0..28 {
        move_data(&vec![(15, x + 1), (15, x)], &mut nodes);
        moves += 1;
        if SHOW_STEPS {
            print_nodes(&nodes, &mut grid, start_node, end_node);
            println!("empty goes from (15,{}) to (15,{})", x, x + 1);
            pause::pause();
        }
    }
    // move the empty cell up
    for y in (0..15).rev() {
        move_data(&vec![(y, 28), (y + 1, 28)], &mut nodes);
        moves += 1;
        if SHOW_STEPS {
            print_nodes(&nodes, &mut grid, start_node, end_node);
            println!("empty goes from ({},28) to ({},28)", y + 1, y);
            pause::pause();
        }
    }
    // then every move of G near S neads 5 moves
    // (G move then the enpty is behind and need 4 moves to go back to left of G)
    for x in (0..28).rev() {
        move_data(&vec![(0, x), (0, x + 1)], &mut nodes);
        end_node = (0, x + 1);
        moves += 5;
        if SHOW_STEPS {
            print_nodes(&nodes, &mut grid, start_node, end_node);
            println!("empty goes from (0,{}) to (0,{})", x + 1, x);
            pause::pause();
        }
    }
    // then finally G needs one move
    end_node = (0, 0);
    moves += 1;
    if SHOW_STEPS {
        print_nodes(&nodes, &mut grid, start_node, end_node);
    }

    moves
}

fn print_nodes(
    nodes: &HashMap<(usize, usize), Node>,
    grid: &mut Grid2D,
    start_node: (usize, usize),
    end_node: (usize, usize),
) {
    for ((y, x), node) in nodes {
        let avail_neighbours = get_avail_neighbours(*y, *x, node, nodes, grid);
        if avail_neighbours > 0 {
            grid.grid[*y][*x] = format!("{}", avail_neighbours).bytes().next().unwrap() as char;
        } else if node.used > 400 {
            grid.grid[*y][*x] = '#';
        } else {
            grid.grid[*y][*x] = '.';
        }
    }
    grid.grid[start_node.0][start_node.1] = 'S';
    grid.grid[end_node.0][end_node.1] = 'G';
    grid.print();
}

fn move_data(moves: &Vec<(usize, usize)>, nodes: &mut HashMap<(usize, usize), Node>) -> bool {
    let mut i = moves.len() - 1;
    loop {
        {
            let moved_used = nodes.get(&moves[i - 1]).unwrap().used;
            let dest = nodes.get_mut(&moves[i]).unwrap();
            if dest.avail < moved_used {
                println!(
                    "error : no space avail on ({},{}) for data from ({},{})",
                    moves[i - 1].0,
                    moves[i - 1].1,
                    moves[i].0,
                    moves[i].1
                );
                return false;
            }
            dest.used += moved_used;
            dest.avail -= moved_used;
        }
        {
            let moved = nodes.get_mut(&moves[i - 1]).unwrap();
            moved.avail += moved.used;
            moved.used = 0;
        }
        i -= 1;
        if i == 0 {
            return true;
        }
    }
}

///
/// given a node, how many neighbours
/// that are not the current node (the node with the goal data)
/// will accept its data in one move
///
fn get_avail_neighbours(
    y: usize,
    x: usize,
    node: &Node,
    nodes: &HashMap<(usize, usize), Node>,
    grid: &mut Grid2D,
) -> usize {
    let mut count = 0;
    let mut neighbours = Vec::new();
    for direction in [(0_isize, 1_isize), (0, -1), (1, 0), (-1, 0)] {
        if y as isize + direction.0 >= 0
            && x as isize + direction.1 >= 0
            && y as isize + direction.0 < grid.max_l as isize
            && x as isize + direction.1 < grid.max_c as isize
        {
            let ny = (y as isize + direction.0) as usize;
            let nx = (x as isize + direction.1) as usize;
            let neighbour = nodes.get(&(ny, nx)).unwrap();
            if neighbour.avail >= node.used {
                count += 1;
                neighbours.push((ny, nx));
            }
        }
    }
    count
}

fn parse(input: &str) -> (HashMap<(usize, usize), Node>, usize, usize) {
    let mut nodes = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    input.lines().skip(2).for_each(|line| {
        let values = line
            .split(['-', ' ', 'x', 'y', 'T', '%'])
            .filter_map(|x| x.parse::<usize>().ok())
            .collect::<Vec<_>>();
        x = values[0];
        y = values[1];
        let node = Node {
            //size: values[2],
            used: values[3],
            avail: values[4],
            //p_use: values[5],
        };
        nodes.insert((y, x), node);
    });
    (nodes, x + 1, y + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer(include_str!("../assets/day22_input.txt")), 198);
    }
}
