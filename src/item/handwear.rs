use crate::aspects::Aspect;

use serde::{
    Deserialize,
    Serialize,
};

pub struct Handwear<'a> {
    pub display_name: &'a str,
    pub description: &'a str,
    pub fire_defense: u32,
    pub frost_defense: u32,
    pub lightning_defense: u32,
    pub physical_defense: u32,
}

impl<'a> Handwear<'a> {
    pub fn get_defense(&self, aspect: Aspect) -> u32 {
        match aspect {
            Aspect::Fire => self.fire_defense,
            Aspect::Frost => self.frost_defense,
            Aspect::Lightning => self.lightning_defense,
            Aspect::Physical => self.physical_defense,
        }
    }
}

impl<'a> From<HandwearIdentifier> for &Handwear<'a> {
    fn from(identifier: HandwearIdentifier) -> Self {
        match identifier {
            HandwearIdentifier::BreakersHandwraps => &BREAKERS_HANDWRAPS,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum HandwearIdentifier {
    BreakersHandwraps,
}

const BREAKERS_HANDWRAPS: Handwear = Handwear {
    display_name: "Breakers Handwraps",
    description: "",
    fire_defense: 0,
    frost_defense: 0,
    lightning_defense: 0,
    physical_defense: 1,
};
