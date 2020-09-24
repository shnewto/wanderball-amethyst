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

    // let mut transform = Transform::default();
    // transform.set_translation_xyz(width * 0.5, height * 0.5, 1.0);

    world
        .create_entity()
        .with(Transform::default())
        .with(Camera::standard_2d(view_diameter, view_diameter))
        .with(Parent { entity: parent })
        .build();
}
