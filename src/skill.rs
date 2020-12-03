use crate::effect::Effect;

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Skill {
    pub display_name: String,
    pub description: String,
    pub effects: Vec<Effect>,
}
