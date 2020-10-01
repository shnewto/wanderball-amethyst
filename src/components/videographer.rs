use amethyst::{
    core::transform::Transform,
    ecs::{Component, Entity, VecStorage, World},
    prelude::*,
};

use crate::config::WanderballConfig;
use crate::resources::save::VideographerRecord;
use serde::{Deserialize, Serialize};

/// The entity that holds the camera and moves it when it needs to
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Videographer {
    pub view_width: f32,
    pub view_height: f32,
    pub view_x: f32,
    pub view_y: f32,
}

impl Component for Videographer {
    type Storage = VecStorage<Self>;
}

pub fn load_videographer(world: &mut World, videograhper_record: VideographerRecord) -> Entity {
    world
        .create_entity()
        .with(videograhper_record.videographer.clone())
        .with(videograhper_record.transform.clone())
        .build()
}


pub fn initialize_videographer(world: &mut World) -> Entity {
    let (view_height, view_width) = {
        let config = &world.read_resource::<WanderballConfig>();
        (config.view_height, config.view_width)
    };

    let videographer = Videographer {
        view_height: view_height,
        view_width: view_width,
        view_x: view_width * 0.5,
        view_y: view_height * 0.5,
    };

    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(videographer.view_x, videographer.view_y, 2.0);

    world
        .create_entity()
        .with(videographer)
        .with(local_transform)
        .build()
}
