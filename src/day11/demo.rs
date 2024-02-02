use super::data::*;
use super::functions::*;

pub fn execute_demo(floors: &mut [Vec<Component>]) {
    // elevator starts a ground
    let mut current_floor = 0;
    print_state(floors, current_floor);

    // execute demo
    // "Bring the Hydrogen-compatible Microchip to the second floor, which is safe because it can get power from the Hydrogen Generator:"
    current_floor = do_it(
        floors,
        current_floor,
        1,
        (
            Component {
                chm: Chemical::Hydrogen,
                equ: Equipment::Microchip,
            },
            Component {
                chm: Chemical::None,
                equ: Equipment::None,
            },
        ),
    );

    // "Bring both Hydrogen-related items to the third floor, which is safe because the Hydrogen-compatible microchip is getting power from its generator"
    current_floor = do_it(
        floors,
        current_floor,
        1,
        (
            Component {
                chm: Chemical::Hydrogen,
                equ: Equipment::Microchip,
            },
            Component {
                chm: Chemical::Hydrogen,
                equ: Equipment::Generator,
            },
        ),
    );

    // "Leave the Hydrogen Generator on floor three, but bring the Hydrogen-compatible Microchip back down with you so you can still use the elevator:"
    current_floor = do_it(
        floors,
        current_floor,
        -1,
        (
            Component {
                chm: Chemical::Hydrogen,
                equ: Equipment::Microchip,
            },
            Component {
                chm: Chemical::None,
                equ: Equipment::None,
            },
        ),
    );
    current_floor = do_it(
        floors,
        current_floor,
        -1,
        (
            Component {
                chm: Chemical::Hydrogen,
                equ: Equipment::Microchip,
            },
            Component {
                chm: Chemical::None,
                equ: Equipment::None,
            },
        ),
    );
    // "At the first floor, grab the Lithium-compatible Microchip, which is safe because Microchips don't affect each other:"
    // "Bring both Microchips up one floor, where there is nothing to fry them:
    current_floor = do_it(
        floors,
        current_floor,
        1,
        (
            Component {
                chm: Chemical::Hydrogen,
                equ: Equipment::Microchip,
            },
            Component {
                chm: Chemical::Lithium,
                equ: Equipment::Microchip,
            },
        ),
    );

    // "Bring both Microchips up again to floor three, where they can be temporarily connected to their corresponding generators while the elevator recharges, preventing either of them from being fried:
    current_floor = do_it(
        floors,
        current_floor,
        1,
        (
            Component {
                chm: Chemical::Hydrogen,
                equ: Equipment::Microchip,
            },
            Component {
                chm: Chemical::Lithium,
                equ: Equipment::Microchip,
            },
        ),
    );

    // "Bring both Microchips to the fourth floor:"
    current_floor = do_it(
        floors,
        current_floor,
        1,
        (
            Component {
                chm: Chemical::Hydrogen,
                equ: Equipment::Microchip,
            },
            Component {
                chm: Chemical::Lithium,
                equ: Equipment::Microchip,
            },
        ),
    );

    // "Leave the Lithium-compatible microchip on the fourth floor, but bring the Hydrogen-compatible one so you can still use the elevator; this is safe because although the Lithium Generator is on the destination floor, you can connect Hydrogen-compatible microchip to the Hydrogen Generator there:"
    current_floor = do_it(
        floors,
        current_floor,
        -1,
        (
            Component {
                chm: Chemical::Hydrogen,
                equ: Equipment::Microchip,
            },
            Component {
                chm: Chemical::None,
                equ: Equipment::None,
            },
        ),
    );

    // "Bring both Generators up to the fourth floor, which is safe because you can connect the Lithium-compatible Microchip to the Lithium Generator upon arrival:"
    current_floor = do_it(
        floors,
        current_floor,
        1,
        (
            Component {
                chm: Chemical::Hydrogen,
                equ: Equipment::Generator,
            },
            Component {
                chm: Chemical::Lithium,
                equ: Equipment::Generator,
            },
        ),
    );

    // "Bring the Lithium Microchip with you to the third floor so you can use the elevator:"
    current_floor = do_it(
        floors,
        current_floor,
        -1,
        (
            Component {
                chm: Chemical::Lithium,
                equ: Equipment::Microchip,
            },
            Component {
                chm: Chemical::None,
                equ: Equipment::None,
            },
        ),
    );

    // "Bring both Microchips to the fourth floor:"
    /*current_floor =*/
    do_it(
        floors,
        current_floor,
        1,
        (
            Component {
                chm: Chemical::Hydrogen,
                equ: Equipment::Microchip,
            },
            Component {
                chm: Chemical::Lithium,
                equ: Equipment::Microchip,
            },
        ),
    );
}
