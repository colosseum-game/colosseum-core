pub mod action;
pub mod aspect;
<<<<<<< HEAD

#[path = "generated/attribute.rs"]
pub mod attribute;

#[path = "generated/combat_event.rs"]
pub mod combat_event;

#[path = "generated/combat_state.rs"]
mod combat_state_generated;
pub mod combat_state;

#[path = "generated/combatant.rs"]
mod combatant_generated;
=======
>>>>>>> parent of 87937be... moved to using protobuf and code generation for every domain type
pub mod combatant;
pub mod consumable;
pub mod effect;
pub mod fraction;
<<<<<<< HEAD

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
=======
pub mod lifetime;
pub mod message;
>>>>>>> parent of 87937be... moved to using protobuf and code generation for every domain type
pub mod modifier;
pub mod party;
pub mod skill;
pub mod store;
pub mod targeting;
pub mod weapon;
pub mod wearable;
