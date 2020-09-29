use amethyst::ecs::{Component, VecStorage};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}

impl Component for Rectangle {
    type Storage = VecStorage<Self>;
}

impl Rectangle {
    pub fn new(width: f32, height: f32) -> Rectangle {
        Rectangle { width, height }
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Rectangle { width: 0.0, height: 0.0 }
    }
}
