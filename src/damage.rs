use crate::{
    lifetime::Lifetime,
    math::Fraction,
};

#[derive(Clone, Copy, Debug)]
pub enum DamageAspect {
    Fire,
    Physical,
}

#[derive(Clone, Copy, Debug)]
pub struct Damage {
    pub aspect: DamageAspect,
    pub scaling: Fraction,
}

#[derive(Clone, Copy, Debug)]
pub struct StatusEffect {
    pub aspect: DamageAspect,
    pub scaling: Fraction,
    pub lifetime: Lifetime,
}

#[derive(Clone, Copy, Debug)]
pub struct StatusEffectEntry {
    pub aspect: DamageAspect,
    pub value: u32,
    pub lifetime: Lifetime,
}
