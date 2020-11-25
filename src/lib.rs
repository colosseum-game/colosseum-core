#[path = "generated/aspect.rs"]
pub mod aspect;

#[path = "generated/attribute.rs"]
pub mod attribute;

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

#[path = "generated/modifier.rs"]
pub mod modifier;

#[path = "generated/party.rs"]
pub mod party;

#[path = "generated/skill.rs"]
pub mod skill;

pub mod store;

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
