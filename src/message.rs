use crate::{
    combat_event::CombatEvent,
    combat_state::CombatState,
};

pub enum MessageType {
    CombatEvent(CombatEvent),
    CombatState(CombatState),
}
