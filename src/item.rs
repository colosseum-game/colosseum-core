use crate::{
    bodywear::BodywearIdentifier,
    consumable::ConsumableIdentifier,
    footwear::FootwearIdentifier,
    handwear::HandwearIdentifier,
    headwear::HeadwearIdentifier,
    legwear::LegwearIdentifier,
    weapon::WeaponIdentifier,
};

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Item {
    Bodywear(BodywearIdentifier),
    Consumable(ConsumableIdentifier),
    Footwear(FootwearIdentifier),
    Handwear(HandwearIdentifier),
    Headwear(HeadwearIdentifier),
    Legwear(LegwearIdentifier),
    Weapon(WeaponIdentifier),
}
