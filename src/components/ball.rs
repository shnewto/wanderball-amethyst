use amethyst::{
    core::{transform::Transform},
    assets::Handle,
    prelude::*,
    ecs::{World, Component, VecStorage},
    renderer::{SpriteRender, SpriteSheet},
};

use crate::config::WanderballConfig;

pub struct Ball {
    pub radius: f32,
}

impl Default for Ball {
    fn default() -> Self {
        Ball { radius: 2.0 }
    }
}
impl Component for Ball {
    type Storage = VecStorage<Self>;
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