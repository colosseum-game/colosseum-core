use crate::{
    aspect::Aspect,
    attribute::Attribute,
    combatant::Combatant,
    gender::Gender,
    lifetime::Lifetime,
    modifier::Modifier,
    fraction::Fraction,
};

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Copy, Debug)]
pub enum EffectSource<'a> {
    None,
    Origin,
    Other(&'a Combatant),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SubEffect {
    Damage { aspect: Aspect, scaling: Fraction },
    DOT { aspect: Aspect, scaling: Fraction, lifetime: Lifetime },
    Modifier { modifier: Modifier, attribute: Attribute },
}

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

impl Default for TargetingScheme {
    fn default() -> TargetingScheme {
        TargetingScheme::All
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Effect {
    pub sub_effects: Vec<SubEffect>,
    pub target_flags: Vec<Vec<TargetFlag>>,
    pub targeting_scheme: TargetingScheme,
}
