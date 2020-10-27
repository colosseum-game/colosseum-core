use crate::{
    combatant::{
        Combatant,
        Stat,
    },
    damage::{
        Damage,
        StatusEffect,
    },
    modifiers::Modifier,
};

#[derive(Clone, Copy, Debug)]
pub enum EffectSource<'a> {
    None,
    Origin,
    Other(&'a Combatant<'a>),
}

#[derive(Debug)]
pub enum Effect {
    Damage(Damage),
    Modifier(Modifier, Stat),
    StatusEffect(StatusEffect),
}
