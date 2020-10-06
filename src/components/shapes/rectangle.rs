use crate::side::Side;
use amethyst::ecs::{Component, VecStorage};
use serde::{Deserialize, Serialize};

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
        Rectangle {
            width: 0.0,
            height: 0.0,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Point2d {
    pub x: f32,
    pub y: f32,
}

pub fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}

pub fn point_outside_rect(
    x: f32,
    y: f32,
    left: f32,
    bottom: f32,
    right: f32,
    top: f32,
) -> Option<Side> {
    if x < left {
        Some(Side::Left)
    } else if x > right {
        Some(Side::Right)
    } else if y < bottom {
        Some(Side::Bottom)
    } else if y > top {
        Some(Side::Top)
    } else {
        None
    }
}
