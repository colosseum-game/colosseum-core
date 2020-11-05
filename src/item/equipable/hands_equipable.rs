use crate::damage::Aspect;

use serde::{
    Deserialize,
    Serialize,
};

pub struct HandsEquipable<'a> {
    pub display_name: &'a str,
    pub description: &'a str,
    pub fire_defense: u32,
    pub frost_defense: u32,
    pub lightning_defense: u32,
    pub physical_defense: u32,
}

impl<'a> HandsEquipable<'a> {
    pub fn get_defense(&self, aspect: Aspect) -> u32 {
        match aspect {
            Aspect::Fire => self.fire_defense,
            Aspect::Frost => self.frost_defense,
            Aspect::Lightning => self.lightning_defense,
            Aspect::Physical => self.physical_defense,
        }
    }
}

impl<'a> From<HandsEquipableIdentifier> for &HandsEquipable<'a> {
    fn from(identifier: HandsEquipableIdentifier) -> Self {
        match identifier {

        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum HandsEquipableIdentifier {

}
