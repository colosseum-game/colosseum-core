use crate::{
    actions::TargetFlag,
    damage::{
        Damage,
        DamageAspect,
        StatusEffect,
    },
    effects::Effect,
    lifetime::Lifetime,
    math::Fraction,
};

use std::fmt;

#[derive(Debug)]
pub struct Item<'a> {
    pub display_name: &'a str,
    pub effects: &'a [Effect],
    pub target_flags: &'a [&'a [TargetFlag]],
    pub target_count: u32,
}

impl<'a> Item<'a> {
    pub fn from_identifier(identifier: ItemIdentifier) -> &'a Self {
        match identifier {
            ItemIdentifier::CrackedBellrootSeed => &CRACKED_BELLROOT_SEED,
            ItemIdentifier::Grenade => &GRENADE,
        }
    }
}

impl<'a> fmt::Display for Item<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ItemIdentifier {
    CrackedBellrootSeed,
    Grenade,
}

const CRACKED_BELLROOT_SEED: Item = Item {
    display_name: "Cracked Bellroot Seed",
    effects: &[
        Effect::Damage(Damage {
            aspect: DamageAspect::Physical,
            scaling: Fraction(3, 1)
        }),
    ],
    target_flags: &[&[TargetFlag::Any]],
    target_count: 3,
};

const GRENADE: Item = Item {
    display_name: "Grenade",
    effects: &[
        Effect::Damage(Damage {
            aspect: DamageAspect::Physical,
            scaling: Fraction(12, 1),
        }),
        Effect::StatusEffect(StatusEffect {
            aspect: DamageAspect::Fire,
            scaling: Fraction(5, 2),
            lifetime: Lifetime::Active(3),
        }),
    ],
    target_flags: &[&[TargetFlag::Any]],
    target_count: 1,
};
