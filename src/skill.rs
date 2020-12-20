use crate::effect::Effect;

use serde::{
    Deserialize,
    Serialize,
};

#[path = "generated/skill.rs"]
mod skill;
pub use skill::SkillIdentifier;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Skill {
    pub display_name: String,
    pub description: String,
    pub effect: Effect,
}
