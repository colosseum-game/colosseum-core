use crate::{
    lifetime::Lifetime,
    math::Fraction,
};

#[derive(Clone, Copy, Debug)]
pub enum ModifierExpression {
    Add(u32),
    Multiply(Fraction),
    Subtract(u32),
}

#[derive(Clone, Copy, Debug)]
pub struct Modifier {
    pub expression: ModifierExpression,
    pub lifetime: Lifetime,
}
