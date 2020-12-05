use crate::target::Target;

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CombatEvent {
    AttackEvent { targets: Vec<Target> },
    ConsumableEvent { consumable_identifier: String, targets: Vec<Target> },
    ForfeitEvent,
    SkipEvent,
    SkillEvent { skill_identifier: String, targets: Vec<Target> },
}
