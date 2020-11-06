use crate::damage::Aspect;

use serde::{
    Deserialize,
    Serialize,
};

pub struct Legwear<'a> {
    pub display_name: &'a str,
    pub description: &'a str,
    pub fire_defense: u32,
    pub frost_defense: u32,
    pub lightning_defense: u32,
    pub physical_defense: u32,
}

impl<'a> Legwear<'a> {
    pub fn get_defense(&self, aspect: Aspect) -> u32 {
        match aspect {
            Aspect::Fire => self.fire_defense,
            Aspect::Frost => self.frost_defense,
            Aspect::Lightning => self.lightning_defense,
            Aspect::Physical => self.physical_defense,
        }
    }
}

impl<'a> From<LegwearIdentifier> for &Legwear<'a> {
    fn from(identifier: LegwearIdentifier) -> Self {
        match identifier {
            LegwearIdentifier::BreakersHaremPants => &BREAKERS_HAREM_PANTS,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum LegwearIdentifier {
    BreakersHaremPants,
}

const BREAKERS_HAREM_PANTS: Legwear = Legwear {
    display_name: "Breakers Harem Pants",
    description: "",
    fire_defense: 0,
    frost_defense: 3,
    lightning_defense: 1,
    physical_defense: 1,
};
