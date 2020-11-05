mod body_equipable;
pub use body_equipable::{
    BodyEquipable,
    BodyEquipableIdentifier,
};

mod feet_equipable;
pub use feet_equipable::{
    FeetEquipable,
    FeetEquipableIdentifier,
};

mod hands_equipable;
pub use hands_equipable::{
    HandsEquipable,
    HandsEquipableIdentifier,
};

mod head_equipable;
pub use head_equipable::{
    HeadEquipable,
    HeadEquipableIdentifier,
};

mod legs_equipable;
pub use legs_equipable::{
    LegsEquipable,
    LegsEquipableIdentifier,
};

mod waist_equipable;
pub use waist_equipable::{
    WaistEquipable,
    WaistEquipableIdentifier,
};

#[derive(Debug)]
pub enum EquipableIdentifier {
    Head(HeadEquipableIdentifier),
    Body(BodyEquipableIdentifier),
    Hands(HandsEquipableIdentifier),
    Waist(WaistEquipableIdentifier),
    Legs(LegsEquipableIdentifier),
    Feet(FeetEquipableIdentifier),
}
