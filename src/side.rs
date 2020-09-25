use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Side {
    Left,
    Bottom,
    Right,
    Top,
}
