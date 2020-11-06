use crate::{
    damage::{
        Damage,
        Aspect,
        StatusEffect,
    },
    effect::{
        Effect,
        SubEffect,
        TargetFlag,
        TargetingScheme,
    },
    lifetime::Lifetime,
    math::Fraction,
};

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug)]
pub struct Action<'a> {
    pub display_name: &'a str,
    pub description: &'a str,
    pub effects: &'a [Effect<'a>],
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

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum ActionIdentifier {
    Attack,
    Scorch,
    Skip,
    Sweep,
}

const ATTACK: Action = Action {
    display_name: "Attack",
    description: "Physically attack any single target on the field",
    effects: &[
        Effect {
            sub_effects: &[
                SubEffect::Damage(Damage {
                    aspect: Aspect::Physical, 
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
    effects: &[
        Effect {
            sub_effects: &[
                SubEffect::StatusEffect(StatusEffect {
                    aspect: Aspect::Fire,
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
    effects: &[],
};

const SWEEP: Action = Action {
    display_name: "Sweep",
    description: "Physically attack any three targets on the field",
    effects: &[
        Effect {
            sub_effects: &[
                SubEffect::Damage(Damage {
                    aspect: Aspect::Physical, 
                    scaling: Fraction(2, 3),
                }),
            ],
            target_flags: &[&[TargetFlag::Any]],
            targeting_scheme: TargetingScheme::MultiTarget(3),
        },
    ],
};
