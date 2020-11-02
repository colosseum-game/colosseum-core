use crate::{
    actions::ActionIdentifier,
    damage::StatusEffectEntry,
    math::Fraction,
    modifiers::{
        Modifier,
        ModifierExpression,
    },
};

use serde::{
    Deserialize,
    Serialize,
};

use std::fmt;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
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
}

const STATS: [Stat; 7] = [
    Stat::Agility, Stat::FireAbsorbtion, Stat::FireAttack,
    Stat::FireDefense, Stat::PhysicalAbsorbtion, Stat::PhysicalAttack,
    Stat::PhysicalDefense
];

impl Stat {
    pub fn iter() -> impl Iterator<Item = Stat> {
        STATS.iter().cloned()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Combatant {
    pub name: String,
    pub gender: Gender,
    pub actions: Vec<ActionIdentifier>,
    pub hp_max: u32,
    pub agility: u32,
    pub fire_attack: u32,
    pub fire_defense: u32,
    pub fire_absorbtion: u32,
    pub physical_attack: u32,
    pub physical_defense: u32,
    pub physical_absorbtion: u32,

    #[serde(skip)] pub hp: u32,
    #[serde(skip)] pub agility_modifiers: Vec<Modifier>,
    #[serde(skip)] pub fire_attack_modifiers: Vec<Modifier>,
    #[serde(skip)] pub fire_defense_modifiers: Vec<Modifier>,
    #[serde(skip)] pub fire_absorbtion_modifiers: Vec<Modifier>,
    #[serde(skip)] pub physical_attack_modifiers: Vec<Modifier>,
    #[serde(skip)] pub physical_defense_modifiers: Vec<Modifier>,
    #[serde(skip)] pub physical_absorbtion_modifiers: Vec<Modifier>,
    #[serde(skip)] pub status_effects: Vec<StatusEffectEntry>,
}

impl Combatant {
    pub fn alive(&self) -> bool {
        self.hp_max > 0 && self.hp > 0
    }

    pub fn dead(&self) -> bool {
        !self.alive()
    }

    pub fn get_stat(&self, stat: Stat) -> u32 {
        let (add, multiply, subtract) = self.get_stat_modifiers(stat).iter().fold((0, Fraction::one(), 0), |acc, modifier|
            match modifier.expression {
                ModifierExpression::Add(value) => (acc.0 + value, acc.1, acc.2),
                ModifierExpression::Multiply(value) => (acc.0, acc.1.multiply(value), acc.2),
                ModifierExpression::Subtract(value) => (acc.0, acc.1, acc.2 + value),
            }
        );

        let mut value = self.get_stat_raw(stat);
        value += add;
        value -= std::cmp::min(value, subtract);
        value *= multiply.0;
        value /= multiply.1;

        value
    }

    pub fn get_stat_raw(&self, stat: Stat) -> u32 {
        match stat {
            Stat::Agility => self.agility,
            Stat::FireAbsorbtion => self.fire_absorbtion,
            Stat::FireAttack => self.fire_attack,
            Stat::FireDefense => self.fire_defense,
            Stat::PhysicalAbsorbtion => self.physical_absorbtion,
            Stat::PhysicalAttack => self.physical_attack,
            Stat::PhysicalDefense => self.physical_defense,
        }
    }

    pub fn get_stat_modifiers(&self, stat: Stat) -> &Vec<Modifier> {
        match stat {
            Stat::Agility => &self.agility_modifiers,
            Stat::FireAbsorbtion => &self.fire_absorbtion_modifiers,
            Stat::FireAttack => &self.fire_attack_modifiers,
            Stat::FireDefense => &self.fire_defense_modifiers,
            Stat::PhysicalAbsorbtion => &self.physical_absorbtion_modifiers,
            Stat::PhysicalAttack => &self.physical_attack_modifiers,
            Stat::PhysicalDefense => &self.physical_defense_modifiers,
        }
    }

    pub fn get_stat_modifiers_mut(&mut self, stat: Stat) -> &mut Vec<Modifier> {
        match stat {
            Stat::Agility => &mut self.agility_modifiers,
            Stat::FireAbsorbtion => &mut self.fire_absorbtion_modifiers,
            Stat::FireAttack => &mut self.fire_attack_modifiers,
            Stat::FireDefense => &mut self.fire_defense_modifiers,
            Stat::PhysicalAbsorbtion => &mut self.physical_absorbtion_modifiers,
            Stat::PhysicalAttack => &mut self.physical_attack_modifiers,
            Stat::PhysicalDefense => &mut self.physical_defense_modifiers,
        }
    }
}

impl fmt::Display for Combatant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
