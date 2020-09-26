use crate::audio::start_audio;
use crate::components::path::initialize_path;
use crate::spritesheet;
use crate::components::ball::initialize_ball;
use crate::camera::initialize_camera;
use crate::components::videographer::initialize_videographer;
use amethyst::prelude::*;

#[derive(Default)]
pub struct Wanderball;

impl SimpleState for Wanderball {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = spritesheet::load_sprite_sheet(world);
        let videographer = initialize_videographer(world);
        initialize_camera(world, videographer);
        initialize_ball(world, sprite_sheet_handle.clone());
        initialize_path(world, sprite_sheet_handle.clone());
        start_audio(world);
    }
}
