use crate::{
    combatant::{
        Combatant,
        Stat,
    },
    damage::{
        ActiveDamage,
        DamageType,
    },
    modifiers::Modifier,
};

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum EffectSource<'a> {
    None,
    Other(&'a Combatant<'a>),
    Target,
}

#[derive(Debug)]
pub enum Effect {
    ActiveDamage(DamageType, u32, u32, u32),
    ActiveModifier(Modifier, Stat, u32),
    Damage(DamageType, u32, u32),
    Modifier(Modifier, Stat),
}
