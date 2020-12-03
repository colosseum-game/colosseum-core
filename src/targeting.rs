use crate::{
    combatant::Combatant,
    effect::EffectSource,
    gender::Gender,
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
