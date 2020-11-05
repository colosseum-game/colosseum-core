use crate::{
    lifetime::Lifetime,
    math::Fraction,
};

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Aspect {
    Fire,
    Frost,
    Lightning,
    Physical,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Damage {
    pub aspect: Aspect,
    pub scaling: Fraction,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct StatusEffect {
    pub aspect: Aspect,
    pub scaling: Fraction,
    pub lifetime: Lifetime,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct StatusEffectEntry {
    pub aspect: Aspect,
    pub value: u32,
    pub lifetime: Lifetime,
}
