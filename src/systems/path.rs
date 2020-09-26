use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Read, ReadStorage, System, SystemData, WriteStorage},
};

use crate::components::path::{Path, PathTile};
use crate::config::WanderballConfig;

#[derive(SystemDesc)]
pub struct PathSystem;

impl<'s> System<'s> for PathSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Path>,
        Read<'s, WanderballConfig>,
    );

    fn run(&mut self, (mut _transforms, _paths, _config): Self::SystemData) {}
}

#[derive(SystemDesc)]
pub struct PathTileSystem;

impl<'s> System<'s> for PathTileSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, PathTile>,
        Read<'s, WanderballConfig>,
    );

    fn run(&mut self, (mut _transforms, _tiles, _config): Self::SystemData) {}
}
