pub mod consumable;
pub use consumable::ConsumableIdentifier;

pub mod equipable;
pub use equipable::EquipableIdentifier;

#[derive(Debug)]
pub enum Item {
    Consumable { identifier: ConsumableIdentifier, count: u32 },
    Equipable(EquipableIdentifier),
}
