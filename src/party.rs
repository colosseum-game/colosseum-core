use crate::{
    combatant::Combatant,
    item::ItemIdentifier,
};

use serde::{
    Deserialize,
    Serialize,
};

pub const INVENTORY_SIZE: usize = 32;

#[derive(Debug, Deserialize, Serialize)]
pub struct Party {
    pub members: Vec<Combatant>,
    pub inventory: [Option<ItemIdentifier>; INVENTORY_SIZE],
}
