use crate::damage::Aspect;

use serde::{
    Deserialize,
    Serialize,
};

pub struct Footwear<'a> {
    pub display_name: &'a str,
    pub description: &'a str,
    pub fire_defense: u32,
    pub frost_defense: u32,
    pub lightning_defense: u32,
    pub physical_defense: u32,
}

impl<'a> Footwear<'a> {
    pub fn get_defense(&self, aspect: Aspect) -> u32 {
        match aspect {
            Aspect::Fire => self.fire_defense,
            Aspect::Frost => self.frost_defense,
            Aspect::Lightning => self.lightning_defense,
            Aspect::Physical => self.physical_defense,
        }
    }
}

impl<'a> From<FootwearIdentifier> for &Footwear<'a> {
    fn from(identifier: FootwearIdentifier) -> Self {
        match identifier {
            FootwearIdentifier::BreakersSneakers => &BREAKERS_SNEAKERS,
            FootwearIdentifier::PlainThighboots => &PLAIN_THIGHBOOTS,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum FootwearIdentifier {
    BreakersSneakers,
    PlainThighboots,
}

const BREAKERS_SNEAKERS: Footwear = Footwear {
    display_name: "Breakers Sneakers",
    description: "Vintage sneakers with rubber soles.",
    fire_defense: 1,
    frost_defense: 2,
    lightning_defense: 7,
    physical_defense: 1,
};

const PLAIN_THIGHBOOTS: Footwear = Footwear {
    display_name: "Plain Thighboots",
    description: "",
    fire_defense: 1,
    frost_defense: 6,
    lightning_defense: 1,
    physical_defense: 3,
};
