use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::components::ball::Ball;
use crate::config::WanderballConfig;

#[derive(SystemDesc)]
pub struct BallSystem;

impl<'s> System<'s> for BallSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Ball>,
        Read<'s, WanderballConfig>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, balls, config, input): Self::SystemData) {
        let movement_x = input.axis_value("move_x");
        let movement_y = input.axis_value("move_y");
        let fast_movement = input.action_is_down("fast_movement");

        for (transform, _) in (&mut transforms, &balls).join() {
            if let Some(mv_amount) = movement_x {
                let scaled_amount;
                if let Some(true) = fast_movement {
                    scaled_amount = mv_amount as f32 * config.fast_movement_speed;
                } else {
                    scaled_amount = mv_amount as f32 * config.movement_speed;
                }
                let ball_x = transform.translation().x;
                transform.set_translation_x(ball_x + scaled_amount);
            }

            if let Some(mv_amount) = movement_y {
                let scaled_amount;
                if let Some(true) = fast_movement {
                    scaled_amount = mv_amount as f32 * config.fast_movement_speed;
                } else {
                    scaled_amount = mv_amount as f32 * config.movement_speed;
                }
                let ball_y = transform.translation().y;
                transform.set_translation_y(ball_y + scaled_amount);
            }
        }
    }
}
