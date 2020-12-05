use crate::target::Target;

pub enum CombatEvent {
    AttackEvent { targets: Vec<Target> },
    ConsumableEvent { consumable_identifier: String, targets: Vec<Target> },
    ForfeitEvent,
    SkipEvent,
    SkillEvent { skill_identifier: String, targets: Vec<Target> },
}
