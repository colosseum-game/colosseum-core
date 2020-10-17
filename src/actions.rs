use crate::{
    combatant::{
        Gender,
        Stat,
    },
    damage::DamageType,
    effects::Effect,
    modifiers::Modifier,
};

use std::fmt;

#[derive(Debug)]
pub enum TargetFlag {
    Any,
    Gender(Gender),
    Source,
}

#[derive(Debug)]
pub struct SubAction<'a> {
    pub effects: &'a [Effect],
    pub target_flags: &'a [&'a [TargetFlag]],
    pub target_count: u32,
}

#[derive(Debug)]
pub struct Action<'a> {
    pub display_name: &'a str,
    pub sub_actions: &'a [SubAction<'a>],
}

impl<'a> fmt::Display for Action<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name)
    }
}

pub const ATTACK: Action = Action {
    display_name: "Attack",
    sub_actions: &[
        SubAction {
            effects: &[
                Effect::Damage(DamageType::Physical, 1, 1),
            ],
            target_flags: &[&[TargetFlag::Any]],
            target_count: 1,
        },
    ],
};

pub const BEAT_FEMALE: Action = Action {
    display_name: "Beat female",
    sub_actions: &[
        SubAction {
            effects: &[
                Effect::Damage(DamageType::Physical, 1, 1),
            ],
            target_flags: &[&[TargetFlag::Gender(Gender::Female)]],
            target_count: 1,
        },
        SubAction {
            effects: &[
                Effect::ActiveModifier(Modifier::Subtract(5), Stat::PhysicalAttack, 1),
            ],
            target_flags: &[&[TargetFlag::Source]],
            target_count: 1,
        },
    ],
};

pub const SKIP: Action = Action {
    display_name: "Skip",
    sub_actions: &[],
};
