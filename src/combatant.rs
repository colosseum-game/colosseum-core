use crate::{
    actions::Action,
    damage::{
        DamagePerTurn,
        DamageType,
    },
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
    FireAttack,
    FireResistance,
    PhysicalAttack,
    PhysicalResistance,
}

#[derive(Debug)]
pub struct Combatant<'a> {
    pub name: &'a str,
    pub gender: Gender,
    pub actions: &'a [&'a Action<'a>],

    pub hp: u32,
    pub hp_max: u32,
    pub damage_per_turn: Vec<DamagePerTurn>,

    pub agility: u32,
    pub fire_attack: u32,
    pub fire_resistance: u32,
    pub physical_attack: u32,
    pub physical_resistance: u32,

    pub agility_modifiers: Vec<Modifier>,
    pub fire_attack_modifiers: Vec<Modifier>,
    pub fire_resistance_modifiers: Vec<Modifier>,
    pub physical_attack_modifiers: Vec<Modifier>,
    pub physical_resistance_modifiers: Vec<Modifier>,
}

impl<'a> Combatant<'a> {
    pub fn alive(&self) -> bool {
        self.hp_max > 0 && self.hp > 0
    }

    pub fn dead(&self) -> bool {
        !self.alive()
    }

    pub fn get_stat(&self, stat: Stat) -> u32 {
        let (mut stat_value, modifiers) = match stat {
            Stat::Agility => (self.agility, &self.agility_modifiers),
            Stat::FireAttack => (self.fire_attack, &self.fire_attack_modifiers),
            Stat::FireResistance => (self.fire_resistance, &self.fire_attack_modifiers),
            Stat::PhysicalAttack => (self.physical_attack, &self.physical_attack_modifiers),
            Stat::PhysicalResistance => (self.physical_resistance, &self.fire_resistance_modifiers),
        };

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
        match stat {
            Stat::Agility => self.agility,
            Stat::FireAttack => self.fire_attack,
            Stat::FireResistance => self.fire_resistance,
            Stat::PhysicalAttack => self.physical_attack,
            Stat::PhysicalResistance => self.physical_resistance,
        }
    }

    pub fn get_stat_modifiers(&self, stat: Stat) -> &Vec<Modifier> {
        match stat {
            Stat::Agility => &self.agility_modifiers,
            Stat::FireAttack => &self.fire_attack_modifiers,
            Stat::FireResistance => &self.fire_resistance_modifiers,
            Stat::PhysicalAttack => &self.physical_attack_modifiers,
            Stat::PhysicalResistance => &self.physical_resistance_modifiers,
        }
    }
}

impl<'a> fmt::Display for Combatant<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
