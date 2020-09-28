use amethyst::{
    input::{is_close_requested, is_key_down},
    ui::{UiEventType, UiCreator, UiEvent, UiFinder},
    winit::{ VirtualKeyCode},
    prelude::{GameData, Trans, StateData, SimpleTrans, StateEvent, SimpleState, WorldExt},
    ecs::prelude::{Entity},
};
use log;

const BUTTON_RESUME: &str = "resume";
const BUTTON_SAVE: &str = "save";
const BUTTON_LOAD: &str = "load";
const BUTTON_QUIT: &str = "quit";

#[derive(Default, Debug)]
pub struct Menu {
    ui_root: Option<Entity>,
    button_resume: Option<Entity>,
    button_save: Option<Entity>,
    button_load: Option<Entity>,
    button_quit: Option<Entity>,    
}

impl SimpleState for Menu {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.ui_root =
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/menu.ron", ())));
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = state_data;

        // "find" buttons once
        if self.button_resume.is_none()
            || self.button_save.is_none()
            || self.button_load.is_none()
            || self.button_quit.is_none()
        {
            world.exec(|ui_finder: UiFinder<'_>| {
                self.button_resume = ui_finder.find(BUTTON_RESUME);
                self.button_save = ui_finder.find(BUTTON_SAVE);
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
                    return Trans::Pop
                } 
                if Some(target) == self.button_save {
                    log::info!("[Trans::None] save game");
                    return Trans::None
                } 
                if Some(target) == self.button_load  {
                    log::info!("[Trans::None] load game");
                    return Trans::None
                } 
                if Some(target) == self.button_quit {
                    log::info!("[Trans::Quit] quit game");
                    return Trans::Quit
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
        self.button_save = None;
        self.button_load = None;
        self.button_quit = None;
    }
}
