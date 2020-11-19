use crate::targeting::Target;

use serde::{
    Deserialize,
    Serialize,
};

pub mod bodywear;
pub use bodywear::{
    Bodywear,
    BodywearIdentifier,
};

pub mod consumable;
pub use consumable::{
    Consumable,
    ConsumableIdentifier,
};

pub mod equipable;
pub use equipable::{
    Equipable,
    EquipableIdentifier,
};

pub mod footwear;
pub use footwear::{
    Footwear,
    FootwearIdentifier,
};

pub mod handwear;
pub use handwear::{
    Handwear,
    HandwearIdentifier,
};

pub mod headwear;
pub use headwear::{
    Headwear,
    HeadwearIdentifier,
};

pub mod legwear;
pub use legwear::{
    Legwear,
    LegwearIdentifier,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ItemAction {
    Consumable { target_groups: Vec<Vec<Target>> },
    Equipable,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ItemIdentifier {
    Bodywear(BodywearIdentifier),
    Consumable(ConsumableIdentifier),
    Equipable(EquipableIdentifier),
    Footwear(FootwearIdentifier),
    Handwear(HandwearIdentifier),
    Headwear(HeadwearIdentifier),
    Legwear(LegwearIdentifier),
}
