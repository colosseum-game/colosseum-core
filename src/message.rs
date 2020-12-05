use crate::{
    combat_event::CombatEvent,
    combat_state::CombatState,
};

pub enum Message {
    CombatEvent(CombatEvent),
    CombatState(CombatState),
}
