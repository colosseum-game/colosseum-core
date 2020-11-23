use crate::effect::Effect;

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Consumable {
    pub display_name: String,
    pub description: String,
    pub max_count: u32,
    pub use_effects: Vec<Effect>,
}
