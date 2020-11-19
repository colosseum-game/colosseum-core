use crate::{
    aspects::Aspect,
    effect::{
        Effect,
        SubEffect,
    },
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

pub struct Equipable<'a> {
    pub display_name: &'a str,
    pub description: &'a str,
    pub use_effects: &'a [Effect<'a>],
}

impl<'a> From<EquipableIdentifier> for &Equipable<'a> {
    fn from(identifier: EquipableIdentifier) -> Self {
        match identifier {
            EquipableIdentifier::PipeIron => &PIPE_IRON,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum EquipableIdentifier {
    PipeIron,
}

const PIPE_IRON: Equipable = Equipable {
    display_name: "Pipe Iron",
    description: "",
    use_effects: &[
        Effect {
            sub_effects: &[
                SubEffect::Damage {
                    aspect: Aspect::Physical,
                    scaling: Fraction { numerator: 3, denominator: 2 },
                },
            ],
            target_flags: &[&[TargetFlag::Any]],
            targeting_scheme: TargetingScheme::SingleTarget,
        }
    ],
};
