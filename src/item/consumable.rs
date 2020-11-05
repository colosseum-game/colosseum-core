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

use variant_count::VariantCount;

#[derive(Debug)]
pub struct Consumable<'a> {
    pub display_name: &'a str,
    pub description: &'a str,
    pub max_count: u32,
    pub effects: &'a [Effect<'a>],
}

impl<'a> From<ConsumableIdentifier> for &Consumable<'a> {
    fn from(identifier: ConsumableIdentifier) -> Self {
        match identifier {
            ConsumableIdentifier::CrackedBellrootSeed => &CRACKED_BELLROOT_SEED,
            ConsumableIdentifier::Grenade => &GRENADE,
        }
    }
}

#[derive(Clone, Copy, Debug, VariantCount)]
pub enum ConsumableIdentifier {
    CrackedBellrootSeed,
    Grenade,
}

const CRACKED_BELLROOT_SEED: Consumable = Consumable {
    display_name: "Cracked Bellroot Seed",
    description: "",
    max_count: 3,
    effects: &[
        Effect {
            sub_effects: &[
                SubEffect::Damage(Damage {
                    aspect: Aspect::Physical,
                    scaling: Fraction(3, 1)
                }),
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
    effects: &[
        Effect {
            sub_effects: &[
                SubEffect::Damage(Damage {
                    aspect: Aspect::Physical,
                    scaling: Fraction(12, 1),
                }),
                SubEffect::StatusEffect(StatusEffect {
                    aspect: Aspect::Fire,
                    scaling: Fraction(5, 2),
                    lifetime: Lifetime::Active(3),
                }),
            ],
            target_flags: &[&[TargetFlag::Any]],
            targeting_scheme: TargetingScheme::SingleTarget,
        },
    ],
};
