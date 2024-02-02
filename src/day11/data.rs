use std::fmt;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Equipment {
    #[default]
    None,
    Generator,
    Microchip,
}
impl fmt::Display for Equipment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Equipment::Generator => write!(f, "G"),
            Equipment::Microchip => write!(f, "M"),
            Equipment::None => write!(f, "-"),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Chemical {
    #[default]
    None,
    Curium,
    Dilithium,
    Elerium,
    Hydrogen,
    Lithium,
    Plutonium,
    Ruthenium,
    Strontium,
    Thulium,
}
impl fmt::Display for Chemical {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Chemical::Curium => write!(f, "C"),
            Chemical::Dilithium => write!(f, "D"),
            Chemical::Elerium => write!(f, "E"),
            Chemical::Hydrogen => write!(f, "H"),
            Chemical::Lithium => write!(f, "L"),
            Chemical::Plutonium => write!(f, "P"),
            Chemical::Ruthenium => write!(f, "R"),
            Chemical::Strontium => write!(f, "S"),
            Chemical::Thulium => write!(f, "T"),
            Chemical::None => write!(f, "-"),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Component {
    pub chm: Chemical,
    pub equ: Equipment,
}
impl std::fmt::Display for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.chm, self.equ)
    }
}

impl Component {
    pub fn is_none(&self) -> bool {
        if *self
            == (Component {
                chm: Chemical::None,
                equ: Equipment::None,
            })
        {
            return true;
        }
        false
    }

    pub fn is_some(&self) -> bool {
        !self.is_none()
    }
}
