use amethyst::{
    assets::Handle,
    core::transform::Transform,
    ecs::{Component, VecStorage, World},
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use crate::config::WanderballConfig;
use crate::components::shapes::circle::Size;

#[derive(Default)]
pub struct Ball;

impl Component for Ball {
    type Storage = VecStorage<Self>;
}

pub fn initialize_ball(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut local_transform = Transform::default();

    let (ball_radius, view_height, view_width) = {
        let config = &world.read_resource::<WanderballConfig>();
        (config.ball_radius, config.view_height, config.view_width)
    };

    let x = view_width - (view_width * 0.25);
    let y = view_height - (view_height * 0.75);
    let z = 1.0;

    local_transform.set_translation_xyz(x, y, z);

    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);

    world
        .create_entity()
        .with(sprite_render)
        .with(Ball::default())
        .with(Size::new(ball_radius))
        .with(local_transform)
        .build();
}
