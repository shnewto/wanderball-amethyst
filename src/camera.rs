use amethyst::{
    core::{transform::Transform, Parent},
    ecs::{Entity, World},
    prelude::*,
    renderer::Camera,
};

use crate::config::WanderballConfig;

pub fn initialize_camera(world: &mut World, parent: Entity) {
    let (view_height, view_width) = {
        let config = &world.read_resource::<WanderballConfig>();
        (config.view_height, config.view_width)
    };

    let camera = Camera::orthographic(
        -view_width / 2.0,
        view_width / 2.0,
        -view_height / 2.0,
        view_height / 2.0,
        1.0,
        4.0,
    );

    world
        .create_entity()
        .with(Transform::default())
        .with(camera)
        .with(Parent { entity: parent })
        .build();
}
