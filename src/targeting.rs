use crate::{
    combatant::{
        Combatant,
        Gender,
    },
    effect::EffectSource,
};

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub enum TargetingScheme {
    All,
    MultiTarget(usize),
    SingleTarget,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Target {
    pub party_index: usize,
    pub member_index: usize,
}
