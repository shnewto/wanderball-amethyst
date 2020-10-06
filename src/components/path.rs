use amethyst::{
    assets::Handle,
    core::{transform::Transform, Hidden},
    ecs::{Component, VecStorage, World},
    prelude::*,
    renderer::{palette::Srgba, resources::Tint, SpriteRender, SpriteSheet},
};

use rand::Rng;

use crate::components::shapes::rectangle::Rectangle;
use crate::config::WanderballConfig;
use crate::resources::save::PathSegmentRecord;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PathSegment;

impl Component for PathSegment {
    type Storage = VecStorage<Self>;
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Path;

impl Component for Path {
    type Storage = VecStorage<Self>;
}

const UP: u8 = 0;
const LEFT: u8 = 1;
const DOWN: u8 = 2;
const RIGHT: u8 = 3;

pub fn load_path(
    world: &mut World,
    path_segments: Vec<PathSegmentRecord>,
    sprite_sheet_handle: &Handle<SpriteSheet>,
) {
    for segment in &path_segments {
        let segment_render = SpriteRender::new(sprite_sheet_handle.clone(), 1);
        world
            .create_entity()
            .with(segment_render)
            .with(PathSegment)
            .with(segment.rectangle.clone())
            .with(segment.transform.clone())
            .build();
    }
    world.insert(path_segments)
}

pub fn initialize_path(world: &mut World, sprite_sheet_handle: &Handle<SpriteSheet>) {
    let (path_segment_height, path_segment_width, path_length, start_x, start_y, start_z) = {
        let config = &world.read_resource::<WanderballConfig>();
        (
            config.path_segment_height,
            config.path_segment_width,
            config.path_length,
            config.start_x,
            config.start_y,
            config.start_path_z,
        )
    };

    let mut path_segments: Vec<PathSegmentRecord> = vec![];

    let tint = Tint(Srgba::new(1.0, 1.0, 1.0, 1.0)); // white == no tint

    // origin path segment
    let mut x = start_x;
    let mut y = start_y;
    let z = start_z;

    let segment_render = SpriteRender::new(sprite_sheet_handle.clone(), 1);

    let mut first_transform = Transform::default();
    first_transform.set_translation_xyz(x, y, z);
    let rectangle = Rectangle::new(path_segment_width, path_segment_height);
    world
        .create_entity()
        .with(segment_render.clone())
        .with(PathSegment)
        .with(rectangle.clone())
        .with(first_transform.clone())
        .with(tint)
        .build();

    path_segments.push(PathSegmentRecord {
        transform: first_transform,
        rectangle,
    });

    // Rest of path
    let mut rng = rand::thread_rng();
    let mut last_choice = LEFT;
    for _ in 1..path_length {
        let choice = rng.gen_range(0, 4);
        let mut rotation = 0.0f32;
        match choice {
            // it's all random! so if we get into a position where we'd have to
            // write over the last sprite we drew, we'll opt for using the
            // last random direction instead
            UP => {
                rotation = 90.0f32;
                if last_choice == DOWN {
                    y -= path_segment_width;
                    last_choice = DOWN;
                } else if last_choice == LEFT {
                    y += path_segment_height;
                    x = x - path_segment_width + path_segment_height;
                    last_choice = choice;
                } else if last_choice == RIGHT {
                    y += path_segment_height;
                    x = x + path_segment_width - path_segment_height;
                    last_choice = choice;
                } else {
                    y += path_segment_width;
                    last_choice = choice;
                }
            }
            LEFT => {
                if last_choice == UP {
                    y = y + path_segment_width * 0.5 + path_segment_height * 0.5;
                    x = x - path_segment_width * 0.5 + path_segment_height * 0.5;
                    last_choice = choice;
                } else if last_choice == RIGHT {
                    x += path_segment_width;
                    last_choice = RIGHT;
                } else if last_choice == DOWN {
                    y = y - path_segment_width * 0.5 - path_segment_height * 0.5;
                    x = x - path_segment_width * 0.5 + path_segment_height * 0.5;
                    last_choice = choice;
                } else {
                    x -= path_segment_width;
                    last_choice = choice;
                }
            }
            DOWN => {
                rotation = 90.0f32;
                if last_choice == UP {
                    y += path_segment_width;
                    last_choice = UP;
                } else if last_choice == LEFT {
                    y -= path_segment_height;
                    x = x - path_segment_width + path_segment_height;
                    last_choice = choice;
                } else if last_choice == RIGHT {
                    y -= path_segment_height;
                    x = x + path_segment_width - path_segment_height;
                    last_choice = choice;
                } else {
                    y -= path_segment_width;
                    last_choice = choice;
                }
            }
            RIGHT => {
                if last_choice == UP {
                    y = y + path_segment_width * 0.5 + path_segment_height * 0.5;
                    x = x + path_segment_width * 0.5 - path_segment_height * 0.5;
                    last_choice = choice;
                } else if last_choice == LEFT {
                    x -= path_segment_width;
                    last_choice = LEFT;
                } else if last_choice == DOWN {
                    y = y - path_segment_width * 0.5 - path_segment_height * 0.5;
                    x = x + path_segment_width * 0.5 - path_segment_height * 0.5;
                    last_choice = choice;
                } else {
                    x += path_segment_width;
                    last_choice = choice;
                }
            }
            _ => unreachable!(),
        }

        let mut next_transform = Transform::default();
        next_transform.set_translation_xyz(x, y, z);
        next_transform.rotate_2d(rotation.to_radians());

        let rectangle: Rectangle;
        if choice == LEFT || choice == RIGHT {
            rectangle = Rectangle::new(path_segment_width, path_segment_height);
        } else {
            // rotated so we flip width/height to make things easier when we're figuring out how to
            // keep the ball on the path when we want to
            rectangle = Rectangle::new(path_segment_height, path_segment_width);
        }

        world
            .create_entity()
            .with(segment_render.clone())
            .with(PathSegment)
            .with(rectangle.clone())
            .with(next_transform.clone())
            .with(tint)
            .with(Hidden)
            .build();

        path_segments.push(PathSegmentRecord {
            transform: next_transform,
            rectangle,
        });
    }

    world.insert(path_segments);
}
