use crate::{
    combatant::{
        Combatant,
        Stat,
    },
    effects::EffectSource,
};

#[derive(Clone, Copy, Debug)]
pub enum DamageType {
    Fire,
    Physical,
}

#[derive(Clone, Copy, Debug)]
pub struct DamagePerTurn {
    pub damage_type: DamageType,
    pub value: u32,
    pub turns_to_live: u32,
}
