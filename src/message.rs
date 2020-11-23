#[repr(u8)]
pub enum MessageType {
    CombatState,
}

pub trait Message {
    fn message_type() -> MessageType;
}
