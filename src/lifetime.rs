#[derive(Clone, Copy, Debug)]
pub enum Lifetime {
    Active(u32),
    Constant,
}
