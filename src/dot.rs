use crate::{
    aspect::Aspect,
    lifetime::Lifetime,
};

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DOT {
    pub aspect: Aspect,
    pub damage_value: u32,
    pub lifetime: Lifetime,
}
