use crate::effect::Effect;

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Weapon {
    pub display_name: String,
    pub description: String,
    pub effect: Effect,
}
