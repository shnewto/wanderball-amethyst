use amethyst::{ecs::Entity, prelude::*};

use crate::resources::save::GameRecord;
use crate::states::game::Wanderball;
use std::{fs::File, io::Read, path::Path};

#[derive(Default, Debug)]
pub struct Loading {
    ui_root: Option<Entity>,
}

impl SimpleState for Loading {
    fn on_start(&mut self, state_data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = state_data;

        let save_dir = Path::new(".save");

        if save_dir.exists() {
            let game_record: GameRecord;
            let save_file_path = save_dir.join("wanderball-save.json");
            let mut f = File::open(save_file_path).unwrap();
            let mut contents = String::new();
            let _ = f.read_to_string(&mut contents);
            game_record = serde_json::from_str(&contents).unwrap();
            world.insert(game_record);
        }
    }

    fn update(&mut self, _state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::Switch(Box::new(Wanderball::default()))
    }
}
