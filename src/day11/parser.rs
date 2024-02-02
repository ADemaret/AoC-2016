use super::data::*;

pub fn parse(input: &str) -> Vec<Vec<Component>> {
    // parse
    let floors = input
        .lines()
        .map(|line| {
            let mut a_floor = Vec::new();
            let (_, e1) = line.split_once("contains ").unwrap();
            let e2 = e1
                .split([' ', ','])
                .filter(|&str| !str.is_empty() && str != "a" && str != "and")
                .collect::<Vec<&str>>();
            let mut a_component = Component {
                ..Default::default()
            };
            //println!("{:?}", e2);
            let mut pairs = e2.chunks(2);
            let mut a_pair = pairs.next();
            while a_pair.is_some_and(|p| p.len() == 2) {
                match &a_pair.unwrap()[0].chars().next().unwrap() {
                    'c' => a_component.chm = Chemical::Curium,
                    'h' => a_component.chm = Chemical::Hydrogen,
                    'l' => a_component.chm = Chemical::Lithium,
                    'p' => a_component.chm = Chemical::Plutonium,
                    'r' => a_component.chm = Chemical::Ruthenium,
                    's' => a_component.chm = Chemical::Strontium,
                    't' => a_component.chm = Chemical::Thulium,
                    _ => {
                        a_component.chm = Chemical::None;
                        //println!("{:?}", &a_pair.unwrap()[0].chars().next());
                    }
                }
                match &a_pair.unwrap()[1].chars().next().unwrap() {
                    'g' => a_component.equ = Equipment::Generator,
                    'm' => a_component.equ = Equipment::Microchip,
                    _ => a_component.equ = Equipment::None,
                }
                a_floor.push(a_component.clone());
                a_pair = pairs.next();
            }
            for i in (0..a_floor.len()).rev() {
                if a_floor[i].chm == Chemical::None && a_floor[i].equ == Equipment::None {
                    a_floor.remove(i);
                } else if a_floor[i].chm == Chemical::None || a_floor[i].equ == Equipment::None {
                    panic!("should be a completely defined equipment")
                }
            }
            a_floor
        })
        .collect::<Vec<Vec<Component>>>();
    floors
}
