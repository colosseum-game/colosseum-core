use crate::damage::Aspect;

use serde::{
    Deserialize,
    Serialize,
};

pub struct Headwear<'a> {
    pub display_name: &'a str,
    pub description: &'a str,
    pub fire_defense: u32,
    pub frost_defense: u32,
    pub lightning_defense: u32,
    pub physical_defense: u32,
}

impl<'a> Headwear<'a> {
    pub fn get_defense(&self, aspect: Aspect) -> u32 {
        match aspect {
            Aspect::Fire => self.fire_defense,
            Aspect::Frost => self.frost_defense,
            Aspect::Lightning => self.lightning_defense,
            Aspect::Physical => self.physical_defense,
        }
    }
}

impl<'a> From<HeadwearIdentifier> for &Headwear<'a> {
    fn from(identifier: HeadwearIdentifier) -> Self {
        match identifier {

        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum HeadwearIdentifier {

}