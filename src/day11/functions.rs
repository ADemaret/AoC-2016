use super::data::*;
use log::*;

pub fn do_it(
    floors: &mut [Vec<Component>],
    base_floor: usize,
    direction: isize,
    in_elevator: (Component, Component),
) -> usize {
    let mut new_floor = base_floor;
    let mut debug_str = format!("Move {}", in_elevator.0);
    if in_elevator.1.is_some() {
        debug_str.push_str(format!(" and {}", in_elevator.1).as_str());
    }
    debug_str.push_str(
        format!(
            " from {} to {}",
            base_floor,
            base_floor as isize + direction
        )
        .as_str(),
    );
    debug!("{}", debug_str);

    //if is_allowed(floors, base_floor, direction, components) {
    // remove component(s) from base floor
    let index = floors[base_floor]
        .iter()
        .position(|&x| x == in_elevator.0)
        .unwrap();
    floors[base_floor].remove(index);
    floors[(base_floor as isize + direction) as usize].push(in_elevator.0);
    if in_elevator.1
        != (Component {
            chm: Chemical::None,
            equ: Equipment::None,
        })
    {
        let index = floors[base_floor]
            .iter()
            .position(|&x| x == in_elevator.1)
            .unwrap();
        floors[base_floor].remove(index);
        if in_elevator.1.is_some() {
            floors[(base_floor as isize + direction) as usize].push(in_elevator.1);
        }
    }
    //print_state(floors, (base_floor as isize + direction) as usize);

    new_floor = (base_floor as isize + direction) as usize;
    //}
    if new_floor == 3 && is_won(floors) {
        debug!("Won !");
    }
    new_floor
}

pub fn print_state(floors: &[Vec<Component>], elevator: usize) {
    for f in (0..4).rev() {
        let mut debug_str = String::new();
        debug_str.push_str(format!("F{f} ").as_str());
        if elevator == f {
            debug_str.push_str("E ");
        } else {
            debug_str.push_str("  ");
        }
        for c in &floors[f] {
            debug_str.push_str(format!("{} ", c).as_str());
        }
        debug!("{debug_str}");
    }
}

pub fn is_allowed(
    floors: &[Vec<Component>],
    base_floor: usize,
    direction: isize,
    in_elevator: (Component, Component),
) -> bool {
    // check the floor
    let destination_floor = (base_floor as isize + direction) as usize;
    if base_floor as isize + direction < 0 || base_floor as isize + direction > 3 {
        // println!(" => NO : floor doesn't exist");
        return false;
    }

    // check compatibility of components at destination floor
    // "if a chip is ever left in the same area as another RTG, and it's not connected to its own RTG, the chip will be fried"
    let mut components_at_destination = Vec::new();
    for c in &floors[destination_floor] {
        components_at_destination.push(*c);
    }
    components_at_destination.push(in_elevator.0);

    if in_elevator.1.is_some() {
        components_at_destination.push(in_elevator.1);
    }
    // print!("components at destination = ");
    // for c in components_at_destination.iter() {
    //     print!("{} ", c);
    // }
    // println!();
    if !compatible_components(&components_at_destination) {
        return false;
    }

    // in the elevator
    if !compatible_components(&vec![in_elevator.0, in_elevator.1]) {
        println!(
            "{} and {} are not compatible in the elevator",
            in_elevator.0, in_elevator.1
        );
        return false;
    }
    // println!(" => YES");
    true
}

fn compatible_components(components: &Vec<Component>) -> bool {
    // println!("are these components compatible ? :");
    // for c in components {
    //     print!("{} ", c);
    // }
    // println!();
    // --
    for c in components {
        if c.equ == Equipment::Microchip // if a microchip...
            && !components.contains(&Component { // .. that has not its own generator ..
                chm: c.chm,
                equ: Equipment::Generator,
            })
        {
            for d in components {
                // .. find another generator ..
                if d.equ == Equipment::Generator && d.chm != c.chm
                // another chemical
                {
                    // .. it fries
                    // println!(" => NO : {} will react with {}", c, d);
                    return false;
                }
            }
        }
    }
    // println!("YES");
    true
}

///
/// are all floors except 4th empty ?
///
pub fn is_won(floors: &[Vec<Component>]) -> bool {
    for floor in floors.iter().take(3) {
        if !floor.is_empty() {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        // LM only => ok
        assert!(compatible_components(&vec![Component {
            chm: Chemical::Lithium,
            equ: Equipment::Microchip
        }]));
        // LG only => ok
        assert!(compatible_components(&vec![Component {
            chm: Chemical::Lithium,
            equ: Equipment::Generator
        }]));
        // LM & LG => ok (own generator)
        assert!(compatible_components(&vec![
            Component {
                chm: Chemical::Lithium,
                equ: Equipment::Microchip
            },
            Component {
                chm: Chemical::Lithium,
                equ: Equipment::Generator
            }
        ]));
        // PM & LM => ok (M&M)
        assert!(compatible_components(&vec![
            Component {
                chm: Chemical::Plutonium,
                equ: Equipment::Microchip
            },
            Component {
                chm: Chemical::Lithium,
                equ: Equipment::Microchip
            }
        ]));
        // PG & LG => ok (G&G)
        assert!(compatible_components(&vec![
            Component {
                chm: Chemical::Plutonium,
                equ: Equipment::Generator
            },
            Component {
                chm: Chemical::Lithium,
                equ: Equipment::Generator
            }
        ]));
        // PM & LG => NOT ok
        assert!(!compatible_components(&vec![
            Component {
                chm: Chemical::Plutonium,
                equ: Equipment::Microchip
            },
            Component {
                chm: Chemical::Lithium,
                equ: Equipment::Generator
            }
        ]));
        // PM & (LG & LM) => NOT ok (PM will be fried)
        assert!(!compatible_components(&vec![
            Component {
                chm: Chemical::Plutonium,
                equ: Equipment::Microchip
            },
            Component {
                chm: Chemical::Lithium,
                equ: Equipment::Generator
            },
            Component {
                chm: Chemical::Lithium,
                equ: Equipment::Microchip
            }
        ]));
        // PG & (LG & LM) => ok
        assert!(compatible_components(&vec![
            Component {
                chm: Chemical::Plutonium,
                equ: Equipment::Generator
            },
            Component {
                chm: Chemical::Lithium,
                equ: Equipment::Generator
            },
            Component {
                chm: Chemical::Lithium,
                equ: Equipment::Microchip
            }
        ]));
    }
}
