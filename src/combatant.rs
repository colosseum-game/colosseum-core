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
    Agility,
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
    pub fn new() -> Self {
        Combatant {
            name: "Unknown".to_string(),
            gender: Gender::Male,
            actions: vec![],
            hp: 0,
            hp_max: 0,
            active_damage: vec![],
            stats: [ 1, 0, 0, 1, 0 ],
            stat_modifiers: [ vec![], vec![], vec![], vec![], vec![] ],
            active_stat_modifiers: [ vec![], vec![], vec![], vec![], vec![] ],
        }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn with_gender(mut self, gender: Gender) -> Self {
        self.gender = gender;
        self
    }

    pub fn with_actions(mut self, actions: &[&'a Action]) -> Self {
        for action in actions {
            self.actions.push(action);
        }
        self
    }

    pub fn with_hp(mut self, hp: u32, hp_max: u32) -> Self {
        self.hp = std::cmp::min(hp, hp_max);
        self.hp_max = hp_max;
        self
    }

    pub fn with_active_damage(mut self, active_damage: ActiveDamage, turns_to_live: u32) -> Self {
        self.active_damage.push((active_damage, turns_to_live));
        self
    }

    pub fn with_stat(mut self, stat: Stat, value: u32) -> Self {
        self.stats[stat as usize] = value;
        self
    }

    pub fn with_stat_modifier(mut self, stat: Stat, modifier: Modifier) -> Self {
        self.stat_modifiers[stat as usize].push(modifier);
        self
    }

    pub fn with_active_modifier(mut self, stat: Stat, modifier: Modifier, turns_to_live: u32) -> Self {
        self.active_stat_modifiers[stat as usize].push((modifier, turns_to_live));
        self
    }

    pub fn alive(&self) -> bool {
        self.hp_max > 0 && self.hp > 0
    }

    pub fn dead(&self) -> bool {
        !self.alive()
    }

    pub fn get_stat(&self, stat: Stat) -> u32 {
        let mut value = self.stats[stat as usize];

        for modifier in &self.stat_modifiers[stat as usize] {
            match modifier {
                Modifier::Add(x) => value += x, // TODO: overflow checking
                Modifier::Divide(x) => value /= x, // TODO: overflow checking
                Modifier::Multiply(x) => value *= x, // TODO: overflow checking
                Modifier::Subtract(x) => value -= x, // TODO: overflow checking
            }
        };

        for (modifier, _) in &self.active_stat_modifiers[stat as usize] {
            match modifier {
                Modifier::Add(x) => value += x, // TODO: overflow checking
                Modifier::Divide(x) => value /= x, // TODO: overflow checking
                Modifier::Multiply(x) => value *= x, // TODO: overflow checking
                Modifier::Subtract(x) => value -= x, // TODO: overflow checking
            }
        }

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
