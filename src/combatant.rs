use crate::{
    actions::ActionIdentifier,
    damage::StatusEffectEntry,
    math::Fraction,
    modifiers::{
        Modifier,
        ModifierExpression,
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

#[derive(Clone, Debug)]
pub struct Combatant {
    pub name: String,
    pub gender: Gender,
    pub actions: Vec<ActionIdentifier>,

    pub hp: u32,
    pub hp_max: u32,
    pub stats: [u32; Stat::MaxValue as usize],

    pub status_effects: Vec<StatusEffectEntry>,
    pub modifiers: [Vec<Modifier>; Stat::MaxValue as usize],
}

impl Combatant {
    pub fn alive(&self) -> bool {
        self.hp_max > 0 && self.hp > 0
    }

    pub fn dead(&self) -> bool {
        !self.alive()
    }

    pub fn get_stat(&self, stat: Stat) -> u32 {
        let (add, multiply, subtract) = self.modifiers[stat as usize].iter().fold((0, Fraction::one(), 0), |acc, modifier|
            match modifier.expression {
                ModifierExpression::Add(value) => (acc.0 + value, acc.1, acc.2),
                ModifierExpression::Multiply(value) => (acc.0, acc.1.multiply(value), acc.2),
                ModifierExpression::Subtract(value) => (acc.0, acc.1, acc.2 + value),
            }
        );

        let mut value = self.stats[stat as usize];
        value += add;
        value -= std::cmp::min(value, subtract);
        value *= multiply.0;
        value /= multiply.1;

        value
    }

    pub fn get_stat_raw(&self, stat: Stat) -> u32 {
        self.stats[stat as usize]
    }
}

impl fmt::Display for Combatant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
