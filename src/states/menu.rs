use amethyst::{
    ecs::prelude::Entity,
    input::{is_close_requested, is_key_down},
    prelude::{GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans, WorldExt},
    shrev::EventChannel,
    ui::{UiCreator, UiEvent, UiEventType, UiFinder},
    winit::VirtualKeyCode,
    TransEvent,
};
use log;

use crate::resources::save::GameRecord;
use crate::states::game::Wanderball;
use crate::states::loading::Loading;
use crate::states::saving::Saving;

const BUTTON_RESUME: &str = "resume";
const BUTTON_RESTART: &str = "restart";
const BUTTON_SAVE: &str = "save";
const BUTTON_LOAD: &str = "load";
const BUTTON_QUIT: &str = "quit";

#[derive(Default, Debug)]
pub struct Menu {
    ui_root: Option<Entity>,
    button_resume: Option<Entity>,
    button_restart: Option<Entity>,
    button_save: Option<Entity>,
    button_load: Option<Entity>,
    button_quit: Option<Entity>,
}

impl SimpleState for Menu {
    fn on_start(&mut self, state_data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = state_data;

        self.ui_root =
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/menu.ron", ())));
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = state_data;

        // "find" buttons once
        if self.button_resume.is_none()
            || self.button_restart.is_none()
            || self.button_save.is_none()
            || self.button_load.is_none()
            || self.button_quit.is_none()
        {
            world.exec(|ui_finder: UiFinder<'_>| {
                self.button_resume = ui_finder.find(BUTTON_RESUME);
                self.button_restart = ui_finder.find(BUTTON_RESTART);
                self.button_save = ui_finder.find(BUTTON_SAVE);
                self.button_load = ui_finder.find(BUTTON_LOAD);
                self.button_quit = ui_finder.find(BUTTON_QUIT);
            });
        }

        Trans::None
    }

    fn handle_event(
        &mut self,
        state_data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    Trans::Quit
                } else if is_key_down(&event, VirtualKeyCode::Escape) {
                    Trans::Pop
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.button_resume {
                    log::info!("[Trans::Pop] resuming wanderball");
                    return Trans::Pop;
                }

                if Some(target) == self.button_restart {
                    let mut state_transition_event_channel = state_data
                        .world
                        .write_resource::<EventChannel<TransEvent<GameData, StateEvent>>>();

                    if let Some(mut game_record) =
                        state_data.world.try_fetch_mut::<Option<GameRecord>>()
                    {
                        // if we loaded a game this session, restarting will bump into
                        // some already initialized state so we need to clear it out here
                        log::info!("found loaded game state, setting it to None");
                        (*game_record) = None;
                    }

                    log::info!("set up state transitions for a game restart");
                    // first 'Pop' the menu and get us to the game state below it. then when we switch, the old game can clean up it's resources in the on_stop handler before we start a new one.
                    state_transition_event_channel.single_write(Box::new(|| Trans::Pop));
                    state_transition_event_channel
                        .single_write(Box::new(|| Trans::Switch(Box::new(Wanderball::default()))));

                    log::info!("[Trans::None] restart game");
                    return Trans::None;
                }

                if Some(target) == self.button_save {
                    log::info!("[Trans::None] save game");
                    return Trans::Push(Box::new(Saving::default()));
                }
                if Some(target) == self.button_load {
                    let mut state_transition_event_channel = state_data
                        .world
                        .write_resource::<EventChannel<TransEvent<GameData, StateEvent>>>();

                    log::info!("[Trans::None] load game");
                    // this allows us to first 'Pop' the menu and get us to the game state below it. then when we switch, the old game can clean up it's resources in the on_stop handler before we load another one.
                    state_transition_event_channel.single_write(Box::new(|| Trans::Pop));
                    state_transition_event_channel
                        .single_write(Box::new(|| Trans::Switch(Box::new(Loading::default()))));

                    return Trans::None;
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
                .expect("failed to close menu");
        }

        self.ui_root = None;
        self.button_resume = None;
        self.button_restart = None;
        self.button_save = None;
        self.button_load = None;
        self.button_quit = None;
    }
}
