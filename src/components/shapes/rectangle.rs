use amethyst::ecs::{Component, VecStorage};

pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Component for Size {
    type Storage = VecStorage<Self>;
}

impl Size {
    pub fn new(width: f32, height: f32) -> Size {
        Size { width, height }
    }
}

impl Default for Size {
    fn default() -> Self {
        Size { width: 0.0, height: 0.0 }
    }
}
