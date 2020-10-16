#[derive(Clone, Copy, Debug)]
pub enum Modifier {
    Add(u32),
    Divide(u32),
    Multiply(u32),
    Subtract(u32),
}
