use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Lifetime {
    Active(u32),
    Constant,
}
