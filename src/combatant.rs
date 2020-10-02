use crate::actions::Action;

use std::fmt;

#[derive(Debug)]
pub enum Gender {
    None,
    Female,
    Male,
}

#[derive(Debug)]
pub struct Combatant {
    pub name: String,
    pub hp: u32,
    pub hp_max: u32,
    pub physical_attack: u32,
    pub physical_resistance: u32,
    pub intelligence: u32,
    pub speed: u32,
    pub flammability: u32,
    pub damage_over_time: u32,
    pub gender: Gender,
    pub isMiso: bool,
    pub actions: Vec::<Action>,
}

impl Combatant {
    pub fn alive(&self) -> bool {
        self.hp_max > 0 && self.hp > 0
    }

    pub fn dead(&self) -> bool {
        !self.alive()
    }
}

impl fmt::Display for Combatant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
