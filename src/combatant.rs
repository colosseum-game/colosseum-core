use crate::{
    actions::Action,
    damage::ActiveDamage,
    modifiers::Modifier,
};

use std::fmt;

#[derive(Debug)]
pub enum Gender {
    Female,
    Male,
    None,
}

#[derive(Clone, Copy, Debug)]
pub enum Stat {
    Agility = 0,
    FireAttack,
    FireResistance,
    PhysicalAttack,
    PhysicalResistance,

    StatMax,
}

#[derive(Debug)]
pub struct Combatant<'a> {
    pub name: String,
    pub gender: Gender,

    pub actions: Vec<&'a Action<'a>>,

    pub hp: u32,
    pub hp_max: u32,
    pub active_damage: Vec<(ActiveDamage, u32)>,

    pub stats: [u32; Stat::StatMax as usize],
    pub stat_modifiers: [Vec<Modifier>; Stat::StatMax as usize],
    pub active_stat_modifiers: [Vec<(Modifier, u32)>; Stat::StatMax as usize],
}

impl<'a> Combatant<'a> {
    pub fn alive(&self) -> bool {
        self.hp_max > 0 && self.hp > 0
    }

    pub fn dead(&self) -> bool {
        !self.alive()
    }

    pub fn get_stat(&self, stat: Stat) -> u32 {
        let mut stat = self.stats[stat as usize];

        for modifier in &self.stat_modifiers[stat as usize] {
            match modifier {
                Modifier::Add(value) => stat += value, // TODO: overflow checking
                Modifier::Divide(value) => stat /= value, // TODO: overflow checking
                Modifier::Multiply(value) => stat *= value, // TODO: overflow checking
                Modifier::Subtract(value) => stat -= value, // TODO: overflow checking
            }
        };

        for (modifier, _) in &self.active_stat_modifiers[stat as usize] {
            match modifier {
                Modifier::Add(value) => stat += value, // TODO: overflow checking
                Modifier::Divide(value) => stat /= value, // TODO: overflow checking
                Modifier::Multiply(value) => stat *= value, // TODO: overflow checking
                Modifier::Subtract(value) => stat -= value, // TODO: overflow checking
            }
        }

        stat
    }

    pub fn get_stat_raw(&self, stat: Stat) -> u32 {
        self.stats[stat as usize]
    }
}

impl<'a> fmt::Display for Combatant<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
