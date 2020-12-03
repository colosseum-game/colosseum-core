use crate::{
    aspect::Aspect,
    attribute::Attribute,
    combatant::Combatant,
    lifetime::Lifetime,
    modifier::Modifier,
    targeting::{
        TargetFlag,
        TargetingScheme,
    },
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
    Modifier(Modifier, Attribute),
    DamageOverTime { aspect: Aspect, scaling: Fraction, lifetime: Lifetime },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Effect {
    pub sub_effects: Vec<SubEffect>,
    pub target_flags: Vec<Vec<TargetFlag>>,
    pub targeting_scheme: TargetingScheme,
}
