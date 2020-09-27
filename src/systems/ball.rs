use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::components::ball::Ball;
use crate::components::videographer::Videographer;
use crate::config::WanderballConfig;

#[derive(SystemDesc)]
pub struct BallSystem;

impl<'s> System<'s> for BallSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Ball>,
        ReadStorage<'s, Videographer>,
        Read<'s, WanderballConfig>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, balls, videographers, config, input): Self::SystemData) {

        let (mut curr_view_width, mut curr_view_height) = (0.0, 0.0);
        for (videographer, _) in (&videographers, &transforms).join() {
            curr_view_width = videographer.view_width;
            curr_view_height = videographer.view_height;
        }

        let move_speed_for_width = curr_view_width / config.move_factor;
        let fast_move_speed_for_width = curr_view_width / config.fast_move_factor;

        let move_speed_for_height = curr_view_height / config.move_factor;
        let fast_move_speed_for_height = curr_view_height / config.fast_move_factor;

        let movement_x = input.axis_value("move_x");
        let movement_y = input.axis_value("move_y");
        let fast_movement = input.action_is_down("fast_movement");

        for (transform, _) in (&mut transforms, &balls).join() {
            if let Some(mv_amount) = movement_x {
                let scaled_amount;
                if let Some(true) = fast_movement {
                    scaled_amount = mv_amount as f32 * fast_move_speed_for_width;
                } else {
                    scaled_amount = mv_amount as f32 * move_speed_for_width;
                }
                let ball_x = transform.translation().x;
                transform.set_translation_x(ball_x + scaled_amount);
            }

            if let Some(mv_amount) = movement_y {
                let scaled_amount;
                if let Some(true) = fast_movement {
                    scaled_amount = mv_amount as f32 * fast_move_speed_for_height;
                } else {
                    scaled_amount = mv_amount as f32 * move_speed_for_height;
                }
                let ball_y = transform.translation().y;
                transform.set_translation_y(ball_y + scaled_amount);
            }
        }
    }
}
