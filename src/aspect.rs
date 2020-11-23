use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Aspect {
    Fire,
    Frost,
    Lightning,
    Physical,
}
