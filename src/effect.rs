use crate::{
    aspects::Aspect,
    combatant::{
        Attribute,
        Combatant,
    },
    lifetimes::Lifetime,
    modifier::Modifier,
    targeting::{
        TargetFlag,
        TargetingScheme,
    },
    fraction::Fraction,
};

#[derive(Clone, Copy, Debug)]
pub enum EffectSource<'a> {
    None,
    Origin,
    Other(&'a Combatant),
}

#[derive(Debug)]
pub enum SubEffect {
    Damage { aspect: Aspect, scaling: Fraction },
    Modifier(Modifier, Attribute),
    DamageOverTime { aspect: Aspect, scaling: Fraction, lifetime: Lifetime },
}

#[derive(Debug)]
pub struct Effect<'a> {
    pub sub_effects: &'a [SubEffect],
    pub target_flags: &'a [&'a [TargetFlag]],
    pub targeting_scheme: TargetingScheme,
}
