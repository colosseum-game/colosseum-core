#[path = "generated/aspect.rs"]
pub mod aspect;

#[path = "generated/attribute.rs"]
pub mod attribute;

#[path = "generated/combat_event.rs"]
pub mod combat_event;

#[path = "generated/combat_state.rs"]
mod combat_state_generated;
pub mod combat_state;

#[path = "generated/combatant.rs"]
mod combatant_generated;
pub mod combatant;

#[path = "generated/consumable.rs"]
pub mod consumable;

#[path = "generated/dot.rs"]
pub mod dot;

#[path = "generated/effect.rs"]
pub mod effect;

#[path = "generated/fraction.rs"]
mod fraction_generated;
pub mod fraction;

#[path = "generated/gender.rs"]
pub mod gender;

<<<<<<< HEAD
#[path = "generated/message.rs"]
pub mod message;
=======
#[path = "generated/lifetime.rs"]
pub mod lifetime;
>>>>>>> parent of aa1a2de... generated some testing content and fixed store

#[path = "generated/modifier.rs"]
pub mod modifier;

#[path = "generated/party.rs"]
pub mod party;

#[path = "generated/skill.rs"]
pub mod skill;

#[path = "generated/target.rs"]
pub mod target;

#[path = "generated/weapon.rs"]
pub mod weapon;

#[path = "generated/bodywear.rs"]
mod bodywear;
#[path = "generated/footwear.rs"]
mod footwear;
#[path = "generated/handwear.rs"]
mod handwear;
#[path = "generated/headwear.rs"]
mod headwear;
#[path = "generated/legwear.rs"]
mod legwear;
pub mod wearable;
