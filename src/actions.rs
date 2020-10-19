use crate::{
    combatant::{
        Combatant,
        Gender,
    },
    effects::{
        Effect,
        EffectSource,
    },
};

use std::fmt;

#[derive(Debug)]
pub enum TargetFlag {
    Any,
    Gender(Gender),
    Origin,
}

impl TargetFlag {
    pub fn satisfied(&self, target: &Combatant, source: EffectSource) -> bool {
        match *self {
            TargetFlag::Any => true,
            TargetFlag::Gender(gender) => target.gender == gender,
            TargetFlag::Origin => match source { EffectSource::Origin => true, _ => false }
        }
    }
}

#[derive(Debug)]
pub struct SubAction<'a> {
    pub effects: &'a [Effect],
    pub target_flags: &'a [&'a [TargetFlag]],
    pub target_count: u32,
}

#[derive(Debug)]
pub struct Action<'a> {
    pub display_name: &'a str,
    pub sub_actions: &'a [SubAction<'a>],
}

impl<'a> fmt::Display for Action<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name)
    }
}
