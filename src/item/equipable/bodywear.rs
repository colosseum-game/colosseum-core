use crate::damage::Aspect;

use serde::{
    Deserialize,
    Serialize,
};

pub struct Bodywear<'a> {
    pub display_name: &'a str,
    pub description: &'a str,
    pub fire_defense: u32,
    pub frost_defense: u32,
    pub lightning_defense: u32,
    pub physical_defense: u32,
}

impl<'a> Bodywear<'a> {
    pub fn get_defense(&self, aspect: Aspect) -> u32 {
        match aspect {
            Aspect::Fire => self.fire_defense,
            Aspect::Frost => self.frost_defense,
            Aspect::Lightning => self.lightning_defense,
            Aspect::Physical => self.physical_defense,
        }
    }
}

impl<'a> From<BodywearIdentifier> for &Bodywear<'a> {
    fn from(identifier: BodywearIdentifier) -> Self {
        match identifier {
            BodywearIdentifier::BreakersHoodie => &BREAKERS_HOODIE
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum BodywearIdentifier {
    BreakersHoodie,
}

const BREAKERS_HOODIE: Bodywear = Bodywear {
    display_name: "Breakers Hoodie",
    description: "",
    fire_defense: 0,
    frost_defense: 5,
    lightning_defense: 1,
    physical_defense: 2,
};
