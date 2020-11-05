use crate::damage::Aspect;

use serde::{
    Deserialize,
    Serialize,
};

pub struct FeetEquipable<'a> {
    pub display_name: &'a str,
    pub description: &'a str,
    pub fire_defense: u32,
    pub frost_defense: u32,
    pub lightning_defense: u32,
    pub physical_defense: u32,
}

impl<'a> FeetEquipable<'a> {
    pub fn get_defense(&self, aspect: Aspect) -> u32 {
        match aspect {
            Aspect::Fire => self.fire_defense,
            Aspect::Frost => self.frost_defense,
            Aspect::Lightning => self.lightning_defense,
            Aspect::Physical => self.physical_defense,
        }
    }
}

impl<'a> From<FeetEquipableIdentifier> for &FeetEquipable<'a> {
    fn from(identifier: FeetEquipableIdentifier) -> Self {
        match identifier {
            FeetEquipableIdentifier::PlainThighboots => &PLAIN_THIGHBOOTS,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum FeetEquipableIdentifier {
    PlainThighboots,
}

const PLAIN_THIGHBOOTS: FeetEquipable = FeetEquipable {
    display_name: "Plain Thighboots",
    description: "",
    fire_defense: 1,
    frost_defense: 6,
    lightning_defense: 1,
    physical_defense: 3,
};
