use crate::{
    consumable::ConsumableIdentifier,
    skill::SkillIdentifier,
    target::Target,
};

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CombatEvent {
    AttackEvent { source: Target, targets: Vec<Target> },
    ConsumableEvent { source: Target, targets: Vec<Target>, consumable: ConsumableIdentifier },
    SkillEvent { source: Target, targets: Vec<Target>, skill: SkillIdentifier },
    SkipEvent,
}
