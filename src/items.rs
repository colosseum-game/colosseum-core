use crate::{
    actions::TargetFlag,
    damage::DamageType,
    effects::Effect,
};

use std::fmt;

#[derive(Debug)]
pub struct Item<'a> {
    display_name: &'a str,
    effects: &'a [Effect],
    target_flags: &'a [&'a [TargetFlag]],
    target_count: u32,
}

impl<'a> fmt::Display for Item<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name)
    }
}

#[allow(dead_code)]
const GRENADE: Item = Item {
    display_name: "Grenade",
    effects: &[
        Effect::Damage(DamageType::Physical, 12, 1),
        Effect::ActiveDamage(DamageType::Fire, 5, 1, 3),
    ],
    target_flags: &[&[TargetFlag::Any]],
    target_count: 1,
};

#[allow(dead_code)]
const CRACKED_BELLROOT_SEED: Item = Item {
    display_name: "Cracked Bellroot Seed",
    effects: &[
        Effect::Damage(DamageType::Physical, 4, 1),
    ],
    target_flags: &[&[TargetFlag::Any]],
    target_count: 3,
};
