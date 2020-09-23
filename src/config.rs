use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct WanderballConfig {
    pub wanderable_height: f32,
    pub wanderable_width: f32,
    pub ball_radius: f32,
    pub path_height: f32,
    pub path_width: f32,
}

impl Default for WanderballConfig {
    fn default() -> Self {
        WanderballConfig {
            wanderable_height: 100.0,
            wanderable_width: 100.0,
            ball_radius: 2.0,
            path_height: 8.0,
            path_width: 24.0,
        }
    }
}
