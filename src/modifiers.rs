use crate::{
    lifetime::Lifetime,
    math::Fraction,
};

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum ModifierExpression {
    Add(u32),
    Multiply(Fraction),
    Subtract(u32),
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Modifier {
    pub expression: ModifierExpression,
    pub lifetime: Lifetime,
}
