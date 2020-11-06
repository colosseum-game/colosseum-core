mod bodywear;
pub use bodywear::{
    Bodywear,
    BodywearIdentifier,
};

mod footwear;
pub use footwear::{
    Footwear,
    FootwearIdentifier,
};

mod handwear;
pub use handwear::{
    Handwear,
    HandwearIdentifier,
};

mod headwear;
pub use headwear::{
    Headwear,
    HeadwearIdentifier,
};

mod legwear;
pub use legwear::{
    Legwear,
    LegwearIdentifier,
};

// TODO: reconsider this?
//#[derive(Debug)]
//pub enum EquipableIdentifier {
//    Head(HeadEquipableIdentifier),
//    Body(ChestpieceIdentifier),
//    Hands(HandsEquipableIdentifier),
//    Waist(WaistEquipableIdentifier),
//    Legs(LegsEquipableIdentifier),
//    Feet(FeetEquipableIdentifier),
//}
