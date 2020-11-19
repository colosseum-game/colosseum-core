use crate::{
    item::ItemAction,
    targeting::Target,
};

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CombatAction {
    Ability { ability_index: usize, target_groups: Vec<Vec<Target>> },
    Attack { target_groups: Vec<Vec<Target>> },
    EnterCombat,
    ExitCombat,
    Forfeit,
    Item(ItemAction),
    Skip,
}
