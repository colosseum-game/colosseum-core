use crate::aspect::Aspect;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use strum::EnumString;

// THIS IS A GENERATED FILE AND NOT INTENDED FOR EDITING

#[derive(Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Handwear {
    pub display_name: String,
    pub description: String,
    pub fire_defense: u32,
    pub frost_defense: u32,
    pub lightning_defense: u32,
    pub physical_defense: u32,
}

impl Handwear {
    pub fn defense(&self, aspect: Aspect) -> u32 {
        match aspect {
            Aspect::Fire => self.fire_defense,
            Aspect::Frost => self.frost_defense,
            Aspect::Lightning => self.lightning_defense,
            Aspect::Physical => self.physical_defense,
        }
    }
}

impl From<HandwearIdentifier> for &Handwear {
    fn from(from: HandwearIdentifier) -> Self {
        &STORE[&from]
    }
}

#[derive(Copy, Clone, Debug, Deserialize, EnumString, Eq, Hash, PartialEq, Serialize)]
#[repr(u8)]
pub enum HandwearIdentifier {
    #[strum(serialize = "breakers_wraps")] BreakersWraps,
}

const DIR: include_dir::Dir = include_dir::include_dir!("content/handwear");

lazy_static::lazy_static! {
    static ref STORE: HashMap<HandwearIdentifier, Handwear> = {
        let mut hashmap = HashMap::new();
        for file in DIR.files {
            let file_name = file.path().file_stem().unwrap().to_str().unwrap();
            let stored = serde_json::from_str(file.contents_utf8().unwrap()).expect(&format!("failed to deserialize {}", file_name));
            hashmap.insert(HandwearIdentifier::from_str(file_name).unwrap(), stored);
        }

        hashmap
    };
}
