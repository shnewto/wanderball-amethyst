use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct WanderballConfig {
    pub view_height: f32,
    pub view_width: f32,
    pub ball_radius: f32,
    pub path_height: f32,
    pub path_width: f32,
    pub path_length: usize,
    pub move_factor: f32,
    pub fast_move_factor: f32,
    pub zoom_factor: f32,
    pub fast_zoom_factor: f32,
}

impl Default for WanderballConfig {
    fn default() -> Self {
        WanderballConfig {
            view_height: 100.0,
            view_width: 100.0,
            ball_radius: 2.0,
            path_height: 8.0,
            path_width: 24.0,
            path_length: 100,
            move_factor: 0.5,
            fast_move_factor: 1.0,
            zoom_factor: 50.0,
            fast_zoom_factor: 10.0,
        }
    }
}
