#[path = "generated/consumable.rs"]
mod consumable;
pub use consumable::ConsumableIdentifier;

use crate::effect::Effect;

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Consumable {
    pub display_name: String,
    pub description: String,
    pub max_count: u32,
    pub effect: Effect,
}
