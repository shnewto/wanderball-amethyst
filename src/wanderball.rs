use crate::audio::start_audio;
use crate::camera::initialize_camera;
use crate::components::ball::initialize_ball;
use crate::components::path::initialize_path;
use crate::components::videographer::initialize_videographer;
use crate::spritesheet;
use amethyst::{
    input::{is_close_requested, is_key_down},
    prelude::*,
    winit::{VirtualKeyCode},
};
use crate::components::shapes::{circle, rectangle};
use crate::menu::Menu;

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

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    log::info!("[Trans::Quit] quitting wanderball");
                    Trans::Quit
                } else if is_key_down(&event, VirtualKeyCode::Escape) {
                    log::info!("[Trans::Switch] switching to menu");
                    Trans::Push(Box::new(Menu::default()))
                } else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
    }    
}
