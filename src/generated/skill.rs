use super::Skill;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use strum::EnumString;

// THIS IS A GENERATED FILE AND NOT INTENDED FOR EDITING

impl From<SkillIdentifier> for &Skill {
    fn from(from: SkillIdentifier) -> Self {
        &STORE[&from]
    }
}

#[derive(Copy, Clone, Debug, Deserialize, EnumString, Eq, Hash, PartialEq, Serialize)]
#[repr(u8)]
pub enum SkillIdentifier {
    #[strum(serialize = "scorch")] Scorch,
    #[strum(serialize = "sweep")] Sweep,
}

const DIR: include_dir::Dir = include_dir::include_dir!("content/skill");

lazy_static::lazy_static! {
    static ref STORE: HashMap<SkillIdentifier, Skill> = {
        let mut hashmap = HashMap::new();
        for file in DIR.files {
            let file_name = file.path().file_stem().unwrap().to_str().unwrap();
            let stored = serde_json::from_str(file.contents_utf8().unwrap()).expect(&format!("failed to deserialize {}", file_name));
            hashmap.insert(SkillIdentifier::from_str(file_name).unwrap(), stored);
        }

        hashmap
    };
}
