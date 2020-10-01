use crate::components::{ videographer::Videographer, shapes::{ circle::Circle, rectangle::Rectangle}};
use amethyst::{core::Transform, renderer::Camera,};
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

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VideographerRecord {
    pub videographer: Videographer,
    pub transform: Transform,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CameraRecord {
    pub transform: Transform,
    pub camera: Camera, 
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameRecord {
    pub path_segments: Vec<PathSegmentRecord>,
    pub balls: Vec<BallRecord>,
    pub videographer: VideographerRecord,
    pub camera: CameraRecord,
}
