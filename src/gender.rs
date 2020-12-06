use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Gender {
    Female,
    Male,
    None,
}
