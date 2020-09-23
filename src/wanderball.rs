use crate::audio::start_audio;
use crate::spritesheet;
use crate::systems::ball::initialize_ball;
use crate::systems::camera::initialize_camera;
use crate::systems::path::starting_path;
use crate::systems::videographer::initialize_videographer;
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
        starting_path(world, sprite_sheet_handle.clone());
        start_audio(world);
    }
}
