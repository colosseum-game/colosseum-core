use crate::{
    combatant::Combatant,
    item::ConsumableIdentifier,
};

pub const PARTY_SIZE: usize = 4;
pub const CONSUMABLES_SIZE: usize = 16;

pub struct Party {
    pub members: [Option<Combatant>; PARTY_SIZE],
    pub consumables: [Option<ConsumableIdentifier>; CONSUMABLES_SIZE],
}
