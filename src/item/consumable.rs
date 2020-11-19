use crate::{
    aspects::Aspect,
    effect::{
        Effect,
        SubEffect,
    },
    lifetimes::Lifetime,
    targeting::{
        TargetFlag,
        TargetingScheme,
    },
    fraction::Fraction,
};

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug)]
pub struct Consumable<'a> {
    pub display_name: &'a str,
    pub description: &'a str,
    pub max_count: u32,
    pub use_effects: &'a [Effect<'a>],
}

impl<'a> From<ConsumableIdentifier> for &Consumable<'a> {
    fn from(identifier: ConsumableIdentifier) -> Self {
        match identifier {
            ConsumableIdentifier::CrackedBellrootSeed => &CRACKED_BELLROOT_SEED,
            ConsumableIdentifier::Grenade => &GRENADE,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum ConsumableIdentifier {
    CrackedBellrootSeed,
    Grenade,
}

const CRACKED_BELLROOT_SEED: Consumable = Consumable {
    display_name: "Cracked Bellroot Seed",
    description: "",
    max_count: 3,
    use_effects: &[
        Effect {
            sub_effects: &[
                SubEffect::Damage {
                    aspect: Aspect::Physical,
                    scaling: Fraction { numerator: 3, denominator: 1 },
                },
            ],
            target_flags: &[&[TargetFlag::Any]],
            targeting_scheme: TargetingScheme::MultiTarget(3),
        }
    ],
};

const GRENADE: Consumable = Consumable {
    display_name: "Grenade",
    description: "",
    max_count: 2,
    use_effects: &[
        Effect {
            sub_effects: &[
                SubEffect::Damage {
                    aspect: Aspect::Physical,
                    scaling: Fraction { numerator: 12, denominator: 1 },
                },
                SubEffect::DamageOverTime {
                    aspect: Aspect::Fire,
                    scaling: Fraction { numerator: 5, denominator: 2 },
                    lifetime: Lifetime::Active { duration: 3 },
                },
            ],
            target_flags: &[&[TargetFlag::Any]],
            targeting_scheme: TargetingScheme::SingleTarget,
        },
    ],
};
