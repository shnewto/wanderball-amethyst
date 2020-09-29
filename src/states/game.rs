use crate::audio::start_audio;
use crate::camera::initialize_camera;
use crate::components::ball::{initialize_ball, load_ball};
use crate::components::path::{initialize_path, load_path};
use crate::components::videographer::initialize_videographer;
use crate::spritesheet;
use amethyst::{
    input::{is_close_requested, is_key_down},
    prelude::*,
    winit::{VirtualKeyCode},
};
use crate::components::shapes::{circle::Circle, rectangle::Rectangle};
use crate::states::menu::Menu;
use crate::resources::save::{GameRecord, PathSegmentRecord, BallRecord};

#[derive(Default)]
pub struct Wanderball;

impl SimpleState for Wanderball {
    fn on_start(&mut self, state_data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = state_data;

        world.register::<Circle>();
        world.register::<Rectangle>();

        let sprite_sheet_handle = spritesheet::load_sprite_sheet(world);
        let videographer = initialize_videographer(world);
        initialize_camera(world, videographer);
        start_audio(world);

        let mut record_elements: Option<(Vec<BallRecord>, Vec<PathSegmentRecord>)> = None;

        if let Some(game_record) = world.try_fetch::<GameRecord>() {
            record_elements = Some(((*game_record.balls).to_vec(), (*game_record.path_segments).to_vec()))
        }

        if let Some((balls, segments)) = record_elements {
            load_ball(world, balls, &sprite_sheet_handle);
            load_path(world, segments, &sprite_sheet_handle); 
        } else {
            initialize_ball(world, &sprite_sheet_handle);
            initialize_path(world, &sprite_sheet_handle); 
        }
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


