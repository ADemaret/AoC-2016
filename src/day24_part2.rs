use std::{
    collections::{HashMap, VecDeque},
    time::Instant,
};

// personal functions
use crate::utils::grid2d::Grid2D;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 24 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day24_input_demo1.txt");
    let input = include_str!("../assets/day24_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

// 294 too low

fn get_answer(input: &str) -> usize {
    let grid = Grid2D::new(input);
    // grid.print();

    // bfs to get distances between goals
    let mut liaisons = Vec::new();
    let mut nodes = HashMap::new();
    for l in 1..(grid.max_l - 1) {
        for c in 0..(grid.max_c - 1) {
            if !(['.', '#']).contains(&grid.grid[l][c]) {
                liaisons.push(bfs(&grid, (l, c)));
                nodes.insert((l, c), grid.grid[l][c]);
            }
        }
    }

    // tsp to get result
    let mut graph = HashMap::new();
    let mut other = Vec::new();
    for l in &liaisons {
        let n1 = nodes.get(&l[0].0).unwrap().to_digit(10).unwrap() as usize;
        if n1 != 0 {
            other.push(n1);
        }
        for (n, d) in l {
            if *d > 0 {
                let n2 = nodes.get(n).unwrap().to_digit(10).unwrap() as usize;
                graph.insert((n1, n2), *d);
            }
        }
    }
    tsp(&graph, 0, &mut other)
}

///
/// Traveling Salesperson Algorithm
/// or
/// Held–Karp algorithm
///
fn tsp(
    graph: &HashMap<(usize, usize), usize>,
    current_node: usize,
    other: &mut Vec<usize>,
) -> usize {
    //
    if other.is_empty() {
        *graph.get(&(current_node, 0)).unwrap()
    } else {
        let mut dists = Vec::new();
        // pour tous les noeuds voisins
        let other_clone = other.clone();
        for node in other {
            let d1k = graph.get(&(current_node, *node)).unwrap();
            let mut other_minus_k = other_clone.clone();
            let k_index = other_minus_k.iter().position(|&x| x == *node).unwrap();
            other_minus_k.remove(k_index);
            let tsp_k = tsp(graph, *node, &mut other_minus_k);
            // add "back to node 0"
            dists.push(d1k + tsp_k);
        }
        *dists.iter().min().unwrap()
    }
}

fn bfs(grid: &Grid2D, start_node: (usize, usize)) -> Vec<((usize, usize), usize)> {
    let mut dejavu = Vec::new();
    let mut queue = VecDeque::new();
    dejavu.push(start_node);

    let mut ret = Vec::new();
    queue.push_back((start_node, 0)); // first cost is 0

    while let Some((current_node, cost)) = queue.pop_front() {
        // trouvé
        if grid.grid[current_node.0][current_node.1].is_ascii_digit() {
            ret.push((current_node, cost));
        }

        // Check the neighboring nodes for any that we've not visited yet.
        for direction in [(0_isize, 1_isize), (0, -1), (1, 0), (-1, 0)] {
            if current_node.0 as isize + direction.0 > 0
                && current_node.1 as isize + direction.1 > 0
                && current_node.0 as isize + direction.0 < grid.max_l as isize - 1
                && current_node.1 as isize + direction.1 < grid.max_c as isize - 1
            {
                let nl = (current_node.0 as isize + direction.0) as usize;
                let nc = (current_node.1 as isize + direction.1) as usize;
                if grid.grid[nl][nc] != '#' && !dejavu.contains(&(nl, nc)) {
                    dejavu.push((nl, nc));
                    queue.push_back(((nl, nc), cost + 1));
                }
            }
        }
    }
    // not found
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer(include_str!("../assets/day24_input.txt")), 724);
    }
}
