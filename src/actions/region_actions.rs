use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum RegionAction {
    Enter,
    Exit,
    Move,
}
