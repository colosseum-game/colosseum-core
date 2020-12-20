use crate::aspect::Aspect;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use strum::EnumString;

// THIS IS A GENERATED FILE AND NOT INTENDED FOR EDITING

#[derive(Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Legwear {
    pub display_name: String,
    pub description: String,
    pub fire_defense: u32,
    pub frost_defense: u32,
    pub lightning_defense: u32,
    pub physical_defense: u32,
}

impl Legwear {
    pub fn defense(&self, aspect: Aspect) -> u32 {
        match aspect {
            Aspect::Fire => self.fire_defense,
            Aspect::Frost => self.frost_defense,
            Aspect::Lightning => self.lightning_defense,
            Aspect::Physical => self.physical_defense,
        }
    }
}

impl From<LegwearIdentifier> for &Legwear {
    fn from(from: LegwearIdentifier) -> Self {
        &STORE[&from]
    }
}

#[derive(Copy, Clone, Debug, Deserialize, EnumString, Eq, Hash, PartialEq, Serialize)]
#[repr(u8)]
pub enum LegwearIdentifier {
    #[strum(serialize = "breakers_harem_pants")] BreakersHaremPants,
}

const DIR: include_dir::Dir = include_dir::include_dir!("content/legwear");

lazy_static::lazy_static! {
    static ref STORE: HashMap<LegwearIdentifier, Legwear> = {
        let mut hashmap = HashMap::new();
        for file in DIR.files {
            let file_name = file.path().file_stem().unwrap().to_str().unwrap();
            let stored = serde_json::from_str(file.contents_utf8().unwrap()).expect(&format!("failed to deserialize {}", file_name));
            hashmap.insert(LegwearIdentifier::from_str(file_name).unwrap(), stored);
        }

        hashmap
    };
}
