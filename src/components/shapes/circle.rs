use amethyst::ecs::{Component, VecStorage};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Circle {
    pub radius: f32,
}

impl Component for Circle {
    type Storage = VecStorage<Self>;
}

impl Circle {
    pub fn new(radius: f32) -> Circle {
        Circle { radius }
    }
}

impl Default for Circle {
    fn default() -> Self {
        Circle { radius: 0.0 }
    }
}
