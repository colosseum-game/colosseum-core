use crate::{
    combatant::Combatant,
};

use serde::{
    Deserialize,
    Serialize,
};

pub const MEMBER_COUNT_MAX: usize = 4;
pub const ITEM_COUNT_MAX: usize = 32;

#[derive(Debug, Deserialize, Serialize)]
pub struct Party {
    pub members: [Option<Combatant>; MEMBER_COUNT_MAX],
    pub inventory: [Option<String>; ITEM_COUNT_MAX],
}
