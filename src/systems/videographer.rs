use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::Camera,
};

use crate::camera::zoom;
use crate::components::ball::Ball;
use crate::components::shapes::rectangle::point_outside_rect;
use crate::components::videographer::Videographer;
use crate::config::WanderballConfig;
use crate::side::Side;

#[derive(SystemDesc, Default)]
pub struct VideographerSystem;

impl<'s> System<'s> for VideographerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Camera>,
        WriteStorage<'s, Videographer>,
        ReadStorage<'s, Ball>,
        Read<'s, WanderballConfig>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(
        &mut self,
        (mut transforms, mut cameras, mut videographers, balls, config, input): Self::SystemData,
    ) {
        let mut ball_x = 0.0;
        let mut ball_y = 0.0;
        // Get the local position of the ball.
        for (_ball, transform) in (&balls, &transforms).join() {
            ball_x = transform.translation().x as f32;
            ball_y = transform.translation().y as f32;
        }

        let left = |videographer: &Videographer| -> f32 {
            videographer.view_x - (videographer.view_width * 0.5)
        };
        let bottom = |videographer: &Videographer| -> f32 {
            videographer.view_y - (videographer.view_height * 0.5)
        };
        let right = |videographer: &Videographer| -> f32 {
            videographer.view_x + (videographer.view_width * 0.5)
        };
        let top = |videographer: &Videographer| -> f32 {
            videographer.view_y + (videographer.view_height * 0.5)
        };

        let fast_zoom = input.action_is_down("fast_movement");

        let (mut curr_view_width, mut curr_view_height) = (0.0, 0.0);
        for (videographer, _) in (&videographers, &transforms).join() {
            curr_view_width = videographer.view_width;
            curr_view_height = videographer.view_height;
        }

        let zoom_factor = (curr_view_height).max(curr_view_width) / config.zoom_factor;
        let fast_zoom_factor = (curr_view_height).max(curr_view_width) / config.fast_zoom_factor;

        let mut height: Option<f32> = None;
        let mut width: Option<f32> = None;
        let mut new_view_height;
        let mut new_view_width;

        for (_, camera) in (&transforms, &mut cameras).join() {
            if let Some(zoom_input) = input.axis_value("zoom") {
                if zoom_input > 0.0 {
                    if let Some(true) = fast_zoom {
                        new_view_height = (curr_view_height - fast_zoom_factor).max(100.0);
                        new_view_width = (curr_view_width - fast_zoom_factor).max(100.0);
                    } else {
                        new_view_height = (curr_view_height - zoom_factor).max(100.0);
                        new_view_width = (curr_view_width - zoom_factor).max(100.0);
                    }
                    zoom(camera, new_view_width, new_view_height);
                    height = Some(new_view_height);
                    width = Some(new_view_width);
                } else if zoom_input < 0.0 {
                    if let Some(true) = fast_zoom {
                        new_view_height = (curr_view_height + fast_zoom_factor).max(100.0);
                        new_view_width = (curr_view_width + fast_zoom_factor).max(100.0);
                    } else {
                        new_view_height = (curr_view_height + zoom_factor).max(100.0);
                        new_view_width = (curr_view_width + zoom_factor).max(100.0);
                    }
                    zoom(camera, new_view_width, new_view_height);
                    height = Some(new_view_height);
                    width = Some(new_view_width);
                }
            }
        }

        for (videographer, transform) in (&mut videographers, &mut transforms).join() {
            if let (Some(new_height), Some(new_width)) = (height, width) {
                videographer.view_height = new_height;
                videographer.view_width = new_width;
            }

            if let Some(side) = point_outside_rect(
                ball_x,
                ball_y,
                left(videographer),
                bottom(videographer),
                right(videographer),
                top(videographer),
            ) {
                let mut new_x = videographer.view_x;
                let mut new_y = videographer.view_y;
                let shift_dist = videographer.view_width;

                match side {
                    Side::Left => {
                        new_x = videographer.view_x - shift_dist;
                    }
                    Side::Bottom => {
                        new_y = videographer.view_y - shift_dist;
                    }
                    Side::Right => {
                        new_x = videographer.view_x + shift_dist;
                    }
                    Side::Top => {
                        new_y = videographer.view_y + shift_dist;
                    }
                }

                transform.set_translation_xyz(new_x, new_y, 2.0);
                // transform.set_translation_y(new_y);
                videographer.view_x = new_x;
                videographer.view_y = new_y;
            }
        }
    }
}
