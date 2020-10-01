use amethyst::{
    ecs::prelude::Entity,
    input::is_close_requested,
    prelude::*,
    ui::{UiCreator, UiEvent, UiEventType, UiFinder},
};

use crate::audio::initialize_audio;
use crate::components::coordinate::CoordinateText;
use crate::states::game::Wanderball;
use crate::states::loading::Loading;

const BUTTON_START: &str = "start";
const BUTTON_LOAD: &str = "load";
const BUTTON_QUIT: &str = "quit";

#[derive(Default, Debug)]
pub struct StartScreen {
    ui_root: Option<Entity>,
    button_start: Option<Entity>,
    button_load: Option<Entity>,
    button_quit: Option<Entity>,
}

impl SimpleState for StartScreen {
    fn on_start(&mut self, state_data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = state_data;

        self.ui_root =
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/start.ron", ())));

        initialize_audio(world);
        let noop_coordinates = world.create_entity().build();
        world.insert(CoordinateText {
            coordinates: noop_coordinates,
        })
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = state_data;

        if self.button_start.is_none() || self.button_load.is_none() || self.button_quit.is_none() {
            world.exec(|ui_finder: UiFinder<'_>| {
                self.button_start = ui_finder.find(BUTTON_START);
                self.button_load = ui_finder.find(BUTTON_LOAD);
                self.button_quit = ui_finder.find(BUTTON_QUIT);
            });
        }

        Trans::None
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.button_start {
                    log::info!("[Trans::None] start game");
                    return Trans::Switch(Box::new(Wanderball::default()));
                }
                if Some(target) == self.button_load {
                    log::info!("[Trans::None] load game");
                    return Trans::Switch(Box::new(Loading::default()));
                }
                if Some(target) == self.button_quit {
                    log::info!("[Trans::Quit] quit game");
                    return Trans::Quit;
                }

                Trans::None
            }
            _ => Trans::None,
        }
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        if let Some(root_entity) = self.ui_root {
            data.world
                .delete_entity(root_entity)
                .expect("failed to close start menu");
        }

        self.ui_root = None;
        self.button_start = None;
        self.button_load = None;
        self.button_quit = None;
    }
}
