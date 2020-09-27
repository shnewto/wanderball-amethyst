use crate::audio::start_audio;
use crate::camera::initialize_camera;
use crate::components::ball::initialize_ball;
use crate::components::path::initialize_path;
use crate::components::videographer::initialize_videographer;
use crate::spritesheet;
use amethyst::prelude::*;
use crate::components::shapes::{circle, rectangle};

#[derive(Default)]
pub struct Wanderball;

impl SimpleState for Wanderball {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<circle::Size>();
        world.register::<rectangle::Size>();

        let sprite_sheet_handle = spritesheet::load_sprite_sheet(world);
        let videographer = initialize_videographer(world);
        initialize_camera(world, videographer);
        initialize_ball(world, sprite_sheet_handle.clone());
        initialize_path(world, sprite_sheet_handle.clone());
        start_audio(world);
    }
}
