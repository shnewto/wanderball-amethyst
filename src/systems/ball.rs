use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::wanderball::{Ball, BALL_RADIUS, WANDERABLE_HEIGHT, WANDERABLE_WIDTH};

#[derive(SystemDesc)]
pub struct BallSystem;

impl<'s> System<'s> for BallSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Ball>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transform, ball, input): Self::SystemData) {
        let movement_x = input.axis_value("move_x");
        let movement_y = input.axis_value("move_y");
        for (_b, t) in (&ball, &mut transform).join() {
            if let Some(mv_amount) = movement_x {
                let scaled_amount = mv_amount as f32;
                let ball_x = t.translation().x;
                t.set_translation_x(
                    (ball_x + scaled_amount)
                        .min(WANDERABLE_WIDTH - BALL_RADIUS)
                        .max(BALL_RADIUS),
                );
            }

            if let Some(mv_amount) = movement_y {
                let scaled_amount = mv_amount as f32;
                let ball_y = t.translation().y;
                t.set_translation_y(
                    (ball_y + scaled_amount)
                        .min(WANDERABLE_HEIGHT - BALL_RADIUS)
                        .max(BALL_RADIUS),
                );
            }
        }
    }
}
