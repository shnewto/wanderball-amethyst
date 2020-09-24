use amethyst::{
    assets::Handle,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Component, DenseVecStorage, Read, ReadStorage, System, SystemData, World},
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use rand::Rng;

use crate::config::WanderballConfig;

pub struct Path {
    pub width: f32,
    pub height: f32,
}

impl Path {
    fn new(width: f32, height: f32) -> Path {
        Path { width, height }
    }
}

impl Component for Path {
    type Storage = DenseVecStorage<Self>;
}

const UP: u8 = 0;
const LEFT: u8 = 1;
const DOWN: u8 = 2;
const RIGHT: u8 = 3;

pub fn initialize_path(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let (view_diameter, path_height, path_width, path_length) = {
        let config = &world.read_resource::<WanderballConfig>();
        (
            config.view_diameter,
            config.path_height,
            config.path_width,
            config.path_length,
        )
    };

    // First sprite of path
    let mut y = view_diameter * 0.25;
    let mut x = 0.0;
    let z: f32 = 0.0;

    let sprite_render = SpriteRender::new(sprite_sheet_handle, 1);

    let mut first_transform = Transform::default();
    first_transform.set_translation_xyz(x, y, z);

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Path::new(path_width, path_height))
        .with(first_transform)
        .build();

    let path_diameter_long = path_width * 0.5;
    let _path_diameter_short = path_height * 0.5;
    // Rest of path
    let mut rng = rand::thread_rng();

    let mut last_choice = LEFT;

    for _ in 0..path_length {
        let sprite = sprite_render.clone();
        let choice = rng.gen_range(0, 3);
        match choice {
            // it's all random! so if we get into a position where we'd have to
            // write over the last sprite we drew, we'll opt for using the
            // last random direction instead
            UP => {
                if last_choice == DOWN {
                    y = y - path_diameter_long;
                } else {
                    y = y + path_diameter_long;
                    last_choice = UP;
                }
            }
            LEFT => {
                if last_choice == RIGHT {
                    x = x + path_diameter_long;
                } else {
                    x = x - path_diameter_long;
                    last_choice = LEFT;
                }
            }
            DOWN => {
                if last_choice == UP {
                    y = y + path_diameter_long;
                } else {
                    y = y - path_diameter_long;
                    last_choice = DOWN;
                }
            }
            RIGHT => {
                if last_choice == LEFT {
                    x = x - path_diameter_long;
                } else {
                    x = x + path_diameter_long;
                    last_choice = RIGHT;
                }
            }
            _ => {
                // this is unreachable so there's gotta be a better way... but for now lets just go left, maybe.
                if last_choice == RIGHT {
                    x = x + path_diameter_long;
                } else {
                    x = x - path_diameter_long;
                    last_choice = LEFT;
                }
            }
        }

        let mut next_transform = Transform::default();
        next_transform.set_translation_xyz(x, y, z);

        if (UP | DOWN) == last_choice {
            next_transform.rotate_2d(90.0f32.to_radians());
        }

        world
            .create_entity()
            .with(sprite)
            .with(Path::new(path_width, path_height))
            .with(next_transform)
            .build();
    }
}

#[derive(SystemDesc)]
pub struct PathSystem;

impl<'s> System<'s> for PathSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Path>,
        Read<'s, WanderballConfig>,
    );

    fn run(&mut self, (mut _transforms, _paths, _config): Self::SystemData) {}
}
