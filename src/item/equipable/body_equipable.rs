use crate::damage::Aspect;

use serde::{
    Deserialize,
    Serialize,
};

pub struct BodyEquipable<'a> {
    pub display_name: &'a str,
    pub description: &'a str,
    pub fire_defense: u32,
    pub frost_defense: u32,
    pub lightning_defense: u32,
    pub physical_defense: u32,
}

impl<'a> BodyEquipable<'a> {
    pub fn get_defense(&self, aspect: Aspect) -> u32 {
        match aspect {
            Aspect::Fire => self.fire_defense,
            Aspect::Frost => self.frost_defense,
            Aspect::Lightning => self.lightning_defense,
            Aspect::Physical => self.physical_defense,
        }
    }
}

impl<'a> From<BodyEquipableIdentifier> for &BodyEquipable<'a> {
    fn from(identifier: BodyEquipableIdentifier) -> Self {
        match identifier {

        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum BodyEquipableIdentifier {

}
