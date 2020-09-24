use amethyst::{
    core::transform::Transform,
    core::Parent,
    ecs::{Entity, World},
    prelude::*,
    renderer::Camera,
};

use crate::config::WanderballConfig;

pub fn initialize_camera(world: &mut World, parent: Entity) {
    let view_diameter = {
        let config = &world.read_resource::<WanderballConfig>();
        config.view_diameter
    };

    world
        .create_entity()
        .with(Transform::default())
        .with(Camera::standard_2d(view_diameter, view_diameter))
        .with(Parent { entity: parent })
        .build();
}
