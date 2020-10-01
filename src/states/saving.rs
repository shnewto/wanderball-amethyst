use amethyst::{
    core::Transform,
    ecs::{Entity, Join},
    prelude::*,
    renderer::Camera,
};

use crate::components::{
    ball::Ball,
    path::PathSegment,
    shapes::{circle::Circle, rectangle::Rectangle},
    videographer::Videographer,
};
use crate::resources::save::{
    BallRecord, CameraRecord, GameRecord, PathSegmentRecord, VideographerRecord,
};
use std::{
    fs::{create_dir, File},
    io::Write,
    path::Path,
};

use log;

#[derive(Default, Debug)]
pub struct Saving {
    ui_root: Option<Entity>,
}

impl SimpleState for Saving {
    fn on_start(&mut self, state_data: StateData<'_, GameData<'_, '_>>) {
        log::info!("start saving state");
        let StateData { world, .. } = state_data;
        // let loader = world.read_resource::<Loader>();

        log::info!("build save");
        if let Some(game_record) = build_save(world) {
            log::info!("built save");

            // maybe save logic needs to start with just a record of high scores...
            // but then prob open up once the world does

            // serialize
            if let Ok(record) = serde_json::to_string(&game_record) {
                log::info!("serialize save");
                let save_dir = Path::new(".save");
                let save_file_path = save_dir.join("wanderball-save.json");

                if save_dir.exists() {
                    if let Ok(mut f) = File::create(save_file_path) {
                        let _ = f.write_all(record.as_bytes());
                        log::info!("serialized save");
                    }
                } else {
                    if let Ok(_) = create_dir(save_dir) {
                        if let Ok(mut f) = File::create(save_file_path) {
                            let _ = f.write_all(record.as_bytes());
                            log::info!("serialized save");
                        }
                    }
                }
            }
        }
    }

    fn update(&mut self, _state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        log::info!("[Trans::Pop] leave saving state");
        Trans::Pop
    }
}

fn build_save(world: &mut World) -> Option<GameRecord> {
    log::info!("collect comonent storages");
    let ball_storage = world.read_storage::<Ball>();
    let circle_storage = world.read_storage::<Circle>();
    let rectangle_storage = world.read_storage::<Rectangle>();
    let path_segment_storage = world.read_storage::<PathSegment>();
    let videographer_storage = world.read_storage::<Videographer>();
    let camera_storage = world.read_storage::<Camera>();
    let transform_storage = world.read_storage::<Transform>();
    log::info!("collected all storages");

    let mut balls: Vec<BallRecord> = vec![];
    let mut path_segments: Vec<PathSegmentRecord> = vec![];
    let mut videographer = VideographerRecord::default();
    let mut maybe_camera: Option<CameraRecord> = None;

    for (_ball, circle, transform) in (&ball_storage, &circle_storage, &transform_storage).join() {
        balls.push(BallRecord {
            transform: transform.clone(),
            circle: circle.clone(),
        })
    }

    for (_segment, rectangle, transform) in (
        &path_segment_storage,
        &rectangle_storage,
        &transform_storage,
    )
        .join()
    {
        path_segments.push(PathSegmentRecord {
            transform: transform.clone(),
            rectangle: rectangle.clone(),
        })
    }

    for (videographer_instance, transform) in (&videographer_storage, &transform_storage).join() {
        videographer = VideographerRecord {
            transform: transform.clone(),
            videographer: videographer_instance.clone(),
        }
    }

    for (camera_instance, transform) in (&camera_storage, &transform_storage).join() {
        maybe_camera = Some(CameraRecord {
            transform: transform.clone(),
            camera: camera_instance.clone(),
        })
    }

    if let Some(camera) = maybe_camera {
        log::info!("construct and return GameRecord");
        Some(GameRecord {
            path_segments,
            balls,
            videographer,
            camera,
        })
    } else {
        log::error!("couldn't find a camera!");
        None
    }
}
