use crate::{
    combatant::Combatant,
    item::Item,
};

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Party {
    pub members: Vec<Combatant>,
    pub inventory: Vec<Item>,
}
