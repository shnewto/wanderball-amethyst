use amethyst::{
    assets::Handle,
    core::{transform::Transform, Parent},
    ecs::{Component, VecStorage, World},
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use rand::Rng;

use crate::config::WanderballConfig;

pub struct PathTile {
    pub width: f32,
    pub height: f32,
}

impl PathTile {
    fn new(width: f32, height: f32) -> PathTile {
        PathTile { width, height }
    }
}

impl Default for PathTile {
    fn default() -> Self {
        PathTile::new(0.0, 0.0)
    }
}

impl Component for PathTile {
    type Storage = VecStorage<Self>;
}

pub struct Path {}

impl Path {
    fn new() -> Path {
        Path {}
    }
}

impl Default for Path {
    fn default() -> Self {
        Path::new()
    }
}

impl Component for Path {
    type Storage = VecStorage<Self>;
}

const UP: u8 = 0;
const LEFT: u8 = 1;
const DOWN: u8 = 2;
const RIGHT: u8 = 3;

pub fn initialize_path(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let (view_height, path_height, path_width, path_length) = {
        let config = &world.read_resource::<WanderballConfig>();
        (
            config.view_height,
            config.path_height,
            config.path_width,
            config.path_length,
        )
    };

    let path = world
        .create_entity()
        .with(Path::default())
        .with(Transform::default())
        .build();

    // origin path tile
    let mut y = view_height * 0.25;
    let mut x = 0.0;
    let z: f32 = 0.0;

    let tile_render = SpriteRender::new(sprite_sheet_handle, 1);

    let mut first_transform = Transform::default();
    first_transform.set_translation_xyz(x, y, z);

    world
        .create_entity()
        .with(tile_render.clone())
        .with(PathTile::new(path_width, path_height))
        .with(first_transform)
        .with(Parent { entity: path })
        .build();

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
                    y = y - path_width;
                    last_choice = DOWN;
                } else if last_choice == LEFT {
                    y = y + path_height;
                    x = x - path_width + path_height;
                    last_choice = choice;
                } else if last_choice == RIGHT {
                    y = y + path_height;
                    x = x + path_width - path_height;
                    last_choice = choice;
                } else {
                    y = y + path_width;
                    last_choice = choice;
                }
            }
            LEFT => {
                if last_choice == UP {
                    y = y + path_width * 0.5 + path_height * 0.5;
                    x = x - path_width * 0.5 + path_height * 0.5;
                    last_choice = choice;
                } else if last_choice == RIGHT {
                    x = x + path_width;
                    last_choice = RIGHT;
                } else if last_choice == DOWN {
                    y = y - path_width * 0.5 - path_height * 0.5;
                    x = x - path_width * 0.5 + path_height * 0.5;
                    last_choice = choice;
                } else {
                    x = x - path_width;
                    last_choice = choice;
                }
            }
            DOWN => {
                rotation = 90.0f32;
                if last_choice == UP {
                    y = y + path_width;
                    last_choice = UP;
                } else if last_choice == LEFT {
                    y = y - path_height;
                    x = x - path_width + path_height;
                    last_choice = choice;
                } else if last_choice == RIGHT {
                    y = y - path_height;
                    x = x + path_width - path_height;
                    last_choice = choice;
                } else {
                    y = y - path_width;
                    last_choice = choice;
                }
            }
            RIGHT => {
                if last_choice == UP {
                    y = y + path_width * 0.5 + path_height * 0.5;
                    x = x + path_width * 0.5 - path_height * 0.5;
                    last_choice = choice;
                } else if last_choice == LEFT {
                    x = x - path_width;
                    last_choice = LEFT;
                } else if last_choice == DOWN {
                    y = y - path_width * 0.5 - path_height * 0.5;
                    x = x + path_width * 0.5 - path_height * 0.5;
                    last_choice = choice;
                } else {
                    x = x + path_width;
                    last_choice = choice;
                }
            }
            _ => unreachable!(),
        }

        let mut next_transform = Transform::default();
        next_transform.set_translation_xyz(x, y, z);
        next_transform.rotate_2d(rotation.to_radians());

        world
            .create_entity()
            .with(tile_render.clone())
            .with(PathTile::new(path_width, path_height))
            .with(next_transform)
            .with(Parent { entity: path })
            .build();
    }
}
