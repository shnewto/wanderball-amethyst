use crate::components::{
    shapes::{circle::Circle, rectangle::{Rectangle}},
    videographer::Videographer,
};

use amethyst::{core::Transform, renderer::Camera};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
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
    pub pedometer: PedometerRecord,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PedometerRecord {
    pub steps: i32,
    pub visited: HashMap<String, ()>,
}
