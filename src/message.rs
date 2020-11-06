use crate::{
    action::ActionIdentifier,
    party::Party,
};

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Deserialize, Serialize)]
pub enum MessageToClient {
    GameState([Party; 2]),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum MessageToServer {
    Action { identifer: ActionIdentifier, target_indices: Vec<usize> },
}
