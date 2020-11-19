use crate::{
    aspects::Aspect,
    effect::{
        Effect,
        SubEffect,
    },
    fraction::Fraction,
    lifetimes::Lifetime,
    targeting::{
        TargetFlag,
        TargetingScheme,
    },
};

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug)]
pub struct Ability<'a> {
    pub display_name: &'a str,
    pub description: &'a str,
    pub effects: &'a [Effect<'a>],
}

impl<'a> From<AbilityIdentifier> for &Ability<'a> {
    fn from(identifier: AbilityIdentifier) -> Self {
        match identifier {
            AbilityIdentifier::Scorch => &SCORCH,
            AbilityIdentifier::Sweep => &SWEEP,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum AbilityIdentifier {
    Scorch,
    Sweep,
}

const SCORCH: Ability = Ability {
    display_name: "Scorch",
    description: "Apply fire damage over time to any single target on the field for 3 turns",
    effects: &[
        Effect {
            sub_effects: &[
                SubEffect::DamageOverTime {
                    aspect: Aspect::Fire,
                    scaling: Fraction { numerator: 2, denominator: 3 },
                    lifetime: Lifetime::Active { duration: 3 },
                },
            ],
            target_flags: &[&[TargetFlag::Any]],
            targeting_scheme: TargetingScheme::SingleTarget,
        },
    ],
};

const SWEEP: Ability = Ability {
    display_name: "Sweep",
    description: "Physically attack any three targets on the field",
    effects: &[
        Effect {
            sub_effects: &[
                SubEffect::Damage {
                    aspect: Aspect::Physical, 
                    scaling: Fraction { numerator: 2, denominator: 3 },
                },
            ],
            target_flags: &[&[TargetFlag::Any]],
            targeting_scheme: TargetingScheme::MultiTarget(3),
        },
    ],
};
