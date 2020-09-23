use crate::audio::start_audio;
use crate::spritesheet;
use crate::systems::ball;
use crate::systems::camera;
use crate::systems::path;
use amethyst::prelude::*;

#[derive(Default)]
pub struct Wanderball;

impl SimpleState for Wanderball {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = spritesheet::load_sprite_sheet(world);

        ball::initialize_ball(world, sprite_sheet_handle.clone());
        path::starting_path(world, sprite_sheet_handle.clone());
        camera::initialize_camera(world);
        start_audio(world);
    }
}
