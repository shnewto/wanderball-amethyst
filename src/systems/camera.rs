use amethyst::{core::Transform, prelude::*, renderer::Camera};

use crate::config::WanderballConfig;

pub fn initialize_camera(world: &mut World) {
    let (height, width) = {
        let config = &world.read_resource::<WanderballConfig>();
        (config.wanderable_width, config.wanderable_height)
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(width * 0.5, height * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(width, height))
        .with(transform)
        .build();
}
