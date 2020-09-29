use amethyst::{
    core::Transform,
};
use crate::components::shapes::rectangle::Rectangle; 
use crate::components::shapes::circle::Circle; 
use serde::{Serialize, Deserialize};

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
