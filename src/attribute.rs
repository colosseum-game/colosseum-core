use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Attribute {
    Agility,
    Dexterity,
    Intelligence,
    Mind,
    Strength,
    Vigor,
    Vitality,
}
