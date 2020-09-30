use crate::components::shapes::circle::Circle;
use crate::components::shapes::rectangle::Rectangle;
use amethyst::core::Transform;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PathSegmentRecord {
    pub transform: Transform,
    pub rectangle: Rectangle,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BallRecord {
    pub transform: Transform,
    pub circle: Circle,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameRecord {
    pub path_segments: Vec<PathSegmentRecord>,
    pub balls: Vec<BallRecord>,
}
