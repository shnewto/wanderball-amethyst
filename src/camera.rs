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

    world
        .create_entity()
        .with(Transform::default())
        .with(Camera::standard_2d(view_width, view_height))
        .with(Parent { entity: parent })
        .build();
}
