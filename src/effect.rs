use crate::{
    combatant::{
        Attribute,
        Combatant,
        Gender,
    },
    damage::{
        Damage,
        StatusEffect,
    },
    modifier::Modifier,
};

#[derive(Clone, Copy, Debug)]
pub enum EffectSource<'a> {
    None,
    Origin,
    Other(&'a Combatant),
}

#[derive(Debug)]
pub enum SubEffect {
    Damage(Damage),
    Modifier(Modifier, Attribute),
    StatusEffect(StatusEffect),
}

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
pub enum TargetingScheme {
    All,
    MultiTarget(usize),
    SingleTarget,
}

#[derive(Debug)]
pub struct Effect<'a> {
    pub sub_effects: &'a [SubEffect],
    pub target_flags: &'a [&'a [TargetFlag]],
    pub targeting_scheme: TargetingScheme,
}
