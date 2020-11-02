use crate::{
    lifetime::Lifetime,
    math::Fraction,
};

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum DamageAspect {
    Fire,
    Physical,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Damage {
    pub aspect: DamageAspect,
    pub scaling: Fraction,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct StatusEffect {
    pub aspect: DamageAspect,
    pub scaling: Fraction,
    pub lifetime: Lifetime,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct StatusEffectEntry {
    pub aspect: DamageAspect,
    pub value: u32,
    pub lifetime: Lifetime,
}
