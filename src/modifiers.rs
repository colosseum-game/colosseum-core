#[derive(Clone, Copy, Debug)]
pub enum ModifierType {
    Add,
    Divide,
    Multiply,
    Subtract,
}

#[derive(Clone, Copy, Debug)]
pub struct Modifier {
    pub modifier_type: ModifierType,
    pub value: u32,
    pub turns_to_live: Option<u32>,
}
