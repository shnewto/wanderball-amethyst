use amethyst::{
    assets::Handle,
    core::Transform,
    derive::SystemDesc,
    ecs::{Component, DenseVecStorage, Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use crate::config::WanderballConfig;

pub struct Ball {
    pub radius: f32,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialize_ball(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut local_transform = Transform::default();

    let (ball_radius, view_height, view_width) = {
        let config = &world.read_resource::<WanderballConfig>();
        (config.ball_radius, config.view_height, config.view_width)
    };

    local_transform.set_translation_xyz(
        view_width - (view_width * 0.25),
        view_height - (view_height * 0.75),
        1.0,
    );

    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);

    world
        .create_entity()
        .with(sprite_render)
        .with(Ball {
            radius: ball_radius,
        })
        .with(local_transform)
        .build();
}

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

        for (transform, _ball) in (&mut transforms, &balls).join() {
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