use amethyst::{
    assets::Handle,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Component, DenseVecStorage, Read, ReadStorage, System, SystemData, World},
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use crate::config::WanderballConfig;
use crate::side::Side;

pub struct Path {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Path {
    fn new(side: Side, width: f32, height: f32) -> Path {
        Path {
            side,
            width,
            height,
        }
    }
}

impl Component for Path {
    type Storage = DenseVecStorage<Self>;
}

pub fn starting_path(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let (wanderable_height, path_height, path_width) = {
        let config = &world.read_resource::<WanderballConfig>();
        (
            config.wanderable_height,
            config.path_height,
            config.path_width,
        )
    };

    let y = wanderable_height * 0.25;
    let x = 0.0;
    let z: f32 = 0.0;

    let mut west_transform = Transform::default();
    west_transform.set_translation_xyz(x, y, z);

    let sprite_render = SpriteRender::new(sprite_sheet_handle, 1);

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Path::new(Side::West, path_width, path_height))
        .with(west_transform)
        .build();
}

#[derive(SystemDesc)]
pub struct PathSystem;

impl<'s> System<'s> for PathSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Path>,
        Read<'s, WanderballConfig>,
    );

    fn run(&mut self, (mut _transforms, _paths, _config): Self::SystemData) {}
}
