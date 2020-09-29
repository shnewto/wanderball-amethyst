use amethyst::{
    ecs::{Entity, Join},
    prelude::*,
    core::Transform,
};

use std::{io::Write, path::Path, fs::{create_dir, File}};
use crate::resources::save::{GameRecord, BallRecord, PathSegmentRecord};
use crate::components::{shapes::{ circle::Circle, rectangle::Rectangle },path::PathSegment, ball::Ball};

#[derive(Default, Debug)]
pub struct Saving {
    ui_root: Option<Entity>,
}

impl SimpleState for Saving {
    fn on_start(&mut self, state_data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = state_data;
        // let loader = world.read_resource::<Loader>();

        let game_record = build_save(world);

        // maybe save logic needs to start with just a record of high scores...
        // but then prob open up once the world does

        // serialize
        if let Ok(record) = serde_json::to_string(&game_record) {
            let save_dir = Path::new(".save");
            let save_file_path = save_dir.join("wanderball-save.json");
    
            if save_dir.exists() {
                if let Ok(mut f) = File::create(save_file_path) {
                    let _ = f.write_all(record.as_bytes());
                }
            } else { 
                if let Ok(_) = create_dir(save_dir) {
                    if let Ok(mut f) = File::create(save_file_path) {
                        let _ = f.write_all(record.as_bytes());
                    }
                }
            }
        }
    }

    fn update(&mut self, _state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::Pop
    }     
}

fn build_save(world: &mut World) -> GameRecord {
    let ball_storage = world.read_storage::<Ball>();
    let circle_storage = world.read_storage::<Circle>();
    let rectangle_storage = world.read_storage::<Rectangle>();
    let path_segment_storage = world.read_storage::<PathSegment>();
    let transform_storage = world.read_storage::<Transform>();

    let mut balls: Vec<BallRecord> = vec![];
    let mut path_segments: Vec<PathSegmentRecord> = vec![];

    for (_ball, circle, transform) in (&ball_storage, &circle_storage, &transform_storage).join() {
        balls.push(BallRecord{
            transform: transform.clone(),
            circle: circle.clone(),
        })
    }

    for (_segment, rectangle, transform) in (&path_segment_storage, &rectangle_storage, &transform_storage).join() {
        path_segments.push(PathSegmentRecord{
            transform: transform.clone(),
            rectangle: rectangle.clone(),
        })
    }

    GameRecord { path_segments, balls }
}