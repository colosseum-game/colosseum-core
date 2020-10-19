use crate::{
    combatant::{
        Combatant,
        Stat,
    },
    damage::DamageType,
    modifiers::Modifier,
};

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum EffectSource<'a> {
    None,
    Origin,
    Other(&'a Combatant<'a>),
}

#[derive(Debug)]
pub enum Effect {
    Damage(DamageType, u32, u32, Option<u32>),
    Modifier(Modifier, Stat),
}
