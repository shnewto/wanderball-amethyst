use amethyst::{
    ecs::prelude::Entity,
    input::{is_close_requested, is_key_down, is_mouse_button_down},
    prelude::*,
    ui::UiCreator,
    winit::{MouseButton, VirtualKeyCode},
};

use crate::audio::initialize_audio;

#[derive(Default, Debug)]
pub struct StartScreen {
    ui_handle: Option<Entity>,
}

impl SimpleState for StartScreen {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.ui_handle =
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/start.ron", ())));

        initialize_audio(world);
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    Trans::Quit
                } else if is_mouse_button_down(&event, MouseButton::Left)
                    || is_key_down(&event, VirtualKeyCode::Return)
                    || is_key_down(&event, VirtualKeyCode::Space)
                {
                    Trans::Switch(Box::new(crate::wanderball::Wanderball::default()))
                } else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        if let Some(root_entity) = self.ui_handle {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to remove StartScreen");
        }

        self.ui_handle = None;
    }
}
