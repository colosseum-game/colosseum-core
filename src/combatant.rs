use crate::{
    actions::Action,
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

#[derive(Debug)]
pub struct Combatant<'a> {
    pub name: &'a str,
    pub gender: Gender,
    pub actions: &'a [&'a Action<'a>],

    pub hp: u32,
    pub hp_max: u32,
    pub stats: [u32; Stat::MaxValue as usize],

    pub status_effects: Vec<StatusEffectEntry>,
    pub modifiers: [Vec<Modifier>; Stat::MaxValue as usize],
}

impl<'a> Combatant<'a> {
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
        value *= multiply.numerator;
        value /= multiply.denominator;

        value
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
