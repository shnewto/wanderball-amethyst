use crate::audio::start_audio;
use crate::camera::{initialize_camera, load_camera};
use crate::components::ball::{initialize_ball, load_ball, Ball};
use crate::components::path::{initialize_path, load_path, PathSegment};
use crate::components::shapes::{circle::Circle, rectangle::Rectangle};
use crate::components::videographer::{initialize_videographer, load_videographer, Videographer};
use crate::components::wanderdata::{init_coordinates, init_pedometer};
use crate::resources::save::GameRecord;
use crate::spritesheet;
use crate::states::menu::Menu;
use amethyst::{
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::Camera,
    winit::VirtualKeyCode,
};

#[derive(Default)]
pub struct Wanderball;

impl SimpleState for Wanderball {
    fn on_start(&mut self, state_data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = state_data;

        world.register::<Circle>();
        world.register::<Rectangle>();

        let sprite_sheet_handle = spritesheet::load_sprite_sheet(world);

        // maybe load game logic
        let mut game_record: Option<GameRecord> = None;
        if let Some(maybe_record) = world.try_fetch::<Option<GameRecord>>() {
            game_record = (*maybe_record).clone();
        }

        // try to load, if we can't... start a new game?
        // should eventually report and prompt for what to do if this happens
        let videographer;
        if let Some(record) = game_record {
            load_path(world, record.path_segments, &sprite_sheet_handle);
            load_ball(world, record.balls, &sprite_sheet_handle);
            videographer = load_videographer(world, record.videographer);
            load_camera(world, record.camera, videographer);
        } else {
            initialize_path(world, &sprite_sheet_handle);
            initialize_ball(world, &sprite_sheet_handle);
            videographer = initialize_videographer(world);
            initialize_camera(world, videographer);
            init_pedometer(world);
        }

        init_coordinates(world);
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

    fn on_stop(&mut self, state_data: StateData<GameData>) {
        let StateData { world, .. } = state_data;
        world.write_storage::<Ball>().clear();
        world.write_storage::<PathSegment>().clear();
        world.write_storage::<Videographer>().clear();
        world.write_storage::<Camera>().clear();
        world.delete_all();
    }
}
