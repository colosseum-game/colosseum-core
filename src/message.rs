use crate::{
    combat_event::CombatEvent,
    combat_state::CombatState,
};

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Message {
    CombatEvent(CombatEvent),
    CombatState(CombatState),
}
