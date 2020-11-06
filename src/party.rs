use crate::{
    combatant::Combatant,
    item::ConsumableIdentifier,
};

use serde::{
    Deserialize,
    Serialize,
};

pub const PARTY_SIZE: usize = 4;
pub const CONSUMABLES_INVENTORY_SIZE: usize = 16;

#[derive(Debug, Deserialize, Serialize)]
pub struct Party {
    pub members: [Option<Combatant>; PARTY_SIZE],
    pub consumables_inventory: [Option<(ConsumableIdentifier, u32)>; CONSUMABLES_INVENTORY_SIZE],
}
