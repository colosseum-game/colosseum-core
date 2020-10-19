use crate::{
    actions::Action,
    damage::DamagePerTurn,
    modifiers::{
        ModifierType,
        Modifier,
    },
};

use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Gender {
    Female,
    Male,
    None,
}

#[derive(Clone, Copy, Debug)]
pub enum Stat {
    Agility,
    FireAbsorbtion,
    FireAttack,
    FireDefense,
    PhysicalAbsorbtion,
    PhysicalAttack,
    PhysicalDefense,

    MaxValue, // TODO: maybe find a better way to do this?
}

#[derive(Debug)]
pub struct Combatant<'a> {
    pub name: &'a str,
    pub gender: Gender,
    pub actions: &'a [&'a Action<'a>],

    pub hp: u32,
    pub hp_max: u32,
    pub damage_per_turn: Vec<DamagePerTurn>,

    pub stats: [u32; Stat::MaxValue as usize],
    pub stat_modifiers: [Vec<Modifier>; Stat::MaxValue as usize],
}

impl<'a> Combatant<'a> {
    pub fn alive(&self) -> bool {
        self.hp_max > 0 && self.hp > 0
    }

    pub fn dead(&self) -> bool {
        !self.alive()
    }

    pub fn get_stat(&self, stat: Stat) -> u32 {
        let (mut stat_value, modifiers) = (self.stats[stat as usize], &self.stat_modifiers[stat as usize]);

        for modifier in modifiers {
            match modifier.modifier_type {
                ModifierType::Add => stat_value += modifier.value, // TODO: overflow checking
                ModifierType::Divide => stat_value /= modifier.value, // TODO: overflow checking
                ModifierType::Multiply => stat_value *= modifier.value, // TODO: overflow checking
                ModifierType::Subtract => stat_value -= modifier.value, // TODO: overflow checking
            }
        };

        stat_value
    }

    pub fn get_stat_raw(&self, stat: Stat) -> u32 {
        self.stats[stat as usize]
    }

    pub fn with_stat(&mut self, stat: Stat, value: u32) -> &mut Self {
        self.stats[stat as usize] = value; 
        self
    }
}

impl<'a> fmt::Display for Combatant<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
