use crate::{
    combatant::{
        Combatant,
        Gender,
    },
    damage::{
        Damage,
        DamageAspect,
        StatusEffect,
    },
    effects::{
        Effect,
        EffectSource,
    },
    lifetime::Lifetime,
    math::Fraction,
};

use std::fmt;

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
pub struct SubAction<'a> {
    pub effects: &'a [Effect],
    pub target_flags: &'a [&'a [TargetFlag]],
    pub targeting_scheme: TargetingScheme,
}

#[derive(Debug)]
pub struct Action<'a> {
    pub display_name: &'a str,
    pub description: &'a str,
    pub sub_actions: &'a [SubAction<'a>],
}

impl<'a> Action<'a> {
    pub fn from_identifier(identifier: ActionIdentifier) -> &'a Self {
        match identifier {
            ActionIdentifier::Attack => &ATTACK,
            ActionIdentifier::Scorch => &SCORCH,
            ActionIdentifier::Skip => &SKIP,
            ActionIdentifier::Sweep => &SWEEP,
        }
    }
}

impl<'a> fmt::Display for Action<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.display_name, self.description)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ActionIdentifier {
    Attack,
    Scorch,
    Skip,
    Sweep,
}

const ATTACK: Action = Action {
    display_name: "Attack",
    description: "Physically attack any single target on the field",
    sub_actions: &[
        SubAction {
            effects: &[
                Effect::Damage(Damage {
                    aspect: DamageAspect::Physical, 
                    scaling: Fraction(1, 1),
                }),
            ],
            target_flags: &[&[TargetFlag::Any]],
            targeting_scheme: TargetingScheme::SingleTarget,
        },
    ],
};

const SCORCH: Action = Action {
    display_name: "Scorch",
    description: "Apply fire damage over time to any single target on the field for 3 turns",
    sub_actions: &[
        SubAction {
            effects: &[
                Effect::StatusEffect(StatusEffect {
                    aspect: DamageAspect::Fire,
                    scaling: Fraction(2, 3),
                    lifetime: Lifetime::Active(3),
                }),
            ],
            target_flags: &[&[TargetFlag::Any]],
            targeting_scheme: TargetingScheme::SingleTarget,
        },
    ],
};

const SKIP: Action = Action {
    display_name: "Skip",
    description: "Do nothing for this turn",
    sub_actions: &[],
};

const SWEEP: Action = Action {
    display_name: "Sweep",
    description: "Physically attack any three targets on the field",
    sub_actions: &[
        SubAction {
            effects: &[
                Effect::Damage(Damage {
                    aspect: DamageAspect::Physical, 
                    scaling: Fraction(2, 3),
                }),
            ],
            target_flags: &[&[TargetFlag::Any]],
            targeting_scheme: TargetingScheme::MultiTarget(3),
        },
    ],
};
