use std::{collections::VecDeque, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 17 - Part 2 --");
    let now = Instant::now();

    let input = "rrrbmfta";
    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> usize {
    // breadth first search
    let start_node = (0, 0);
    let end_node = (3, 3);
    let mut queue = VecDeque::new();
    let mut pathes = Vec::new();

    queue.push_back((start_node, String::new()));
    while let Some((current_node, path)) = queue.pop_front() {
        // trouvé
        if current_node == end_node {
            pathes.push(path.clone());
        } else {
            let udlr = get_open_doors(input, &path);
            if udlr[0] && current_node.0 > 0 {
                let mut new_path = path.clone();
                let new_node = (current_node.0 - 1, current_node.1);
                new_path.push('U');
                queue.push_back((new_node, new_path))
            }
            if udlr[1] && current_node.0 < 3 {
                let mut new_path = path.clone();
                let new_node = (current_node.0 + 1, current_node.1);
                new_path.push('D');
                queue.push_back((new_node, new_path))
            }
            if udlr[2] && current_node.1 > 0 {
                let mut new_path = path.clone();
                let new_node = (current_node.0, current_node.1 - 1);
                new_path.push('L');
                queue.push_back((new_node, new_path))
            }
            if udlr[3] && current_node.1 < 3 {
                let mut new_path = path.clone();
                let new_node = (current_node.0, current_node.1 + 1);
                new_path.push('R');
                queue.push_back((new_node, new_path))
            }
        }
    }
    pathes.iter().map(|str| str.len()).max().unwrap()
}

fn get_open_doors(input: &str, path: &String) -> Vec<bool> {
    let org_str = format!("{input}{path}");
    let md5_str = format!("{:x}", md5::compute(org_str));
    //println!("{md5_str}");
    let udlr = md5_str.chars().take(4).collect::<Vec<_>>();
    udlr.iter()
        .map(|x| matches!(x, 'b' | 'c' | 'd' | 'e' | 'f'))
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_doors() {
        assert_eq!(
            get_open_doors("hijkl", &"".to_string()),
            vec![true, true, true, false]
        );
        assert_eq!(
            get_open_doors("hijkl", &"D".to_string()),
            vec![true, false, true, true]
        );
        assert_eq!(
            get_open_doors("hijkl", &"DU".to_string()),
            vec![false, false, false, true]
        );
        assert_eq!(
            get_open_doors("hijkl", &"DUR".to_string()),
            vec![false, false, false, false]
        );
    }

    #[test]
    fn test_total() {
        assert_eq!(get_answer("ihgpwlah"), 370);
        assert_eq!(get_answer("kglvqrro"), 492);
        assert_eq!(get_answer("ulqzkmiv"), 830);
        assert_eq!(get_answer("rrrbmfta"), 420);
    }
}
