use crate::{
    fraction::Fraction,
    lifetimes::Lifetime,
};

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ModifierExpression {
    Add(u32),
    Multiply(Fraction),
    Subtract(u32),
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Modifier {
    pub expression: ModifierExpression,
    pub lifetime: Lifetime,
}
