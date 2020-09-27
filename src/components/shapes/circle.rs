use amethyst::ecs::{Component, VecStorage};

pub struct Size {
    pub radius: f32,
}

impl Component for Size {
    type Storage = VecStorage<Self>;
}

impl Size {
    pub fn new(radius: f32) -> Size {
        Size { radius}
    }
}

impl Default for Size {
    fn default() -> Self {
        Size { radius: 0.0 }
    }
}