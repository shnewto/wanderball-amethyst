use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use crate::audio::start_audio;

pub const WANDERABLE_HEIGHT: f32 = 100.0;
pub const WANDERABLE_WIDTH: f32 = 100.0;

pub const BALL_RADIUS: f32 = 2.0;

pub const DOOR_HEIGHT: f32 = 8.0;
pub const DOOR_WIDTH: f32 = 2.0;

#[derive(PartialEq, Eq)]
pub enum Wall {
    North,
    South,
    East,
    West,
}

pub struct Door {
    pub wall: Wall,
    pub width: f32,
    pub height: f32,
}

impl Door {
    fn new(wall: Wall) -> Door {
        Door {
            wall,
            width: DOOR_WIDTH,
            height: DOOR_HEIGHT,
        }
    }
}

impl Component for Door {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct Wanderball;

impl SimpleState for Wanderball {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(world);

        world.register::<Door>();

        initialize_doors(world, sprite_sheet_handle.clone());
        initialize_ball(world, sprite_sheet_handle.clone());
        initialize_camera(world);
        start_audio(world);
    }
}

pub struct Ball {
    pub radius: f32,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

fn initialize_ball(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(
        WANDERABLE_WIDTH - (WANDERABLE_WIDTH * 0.25),
        WANDERABLE_HEIGHT - (WANDERABLE_HEIGHT * 0.75),
        0.0,
    );

    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);

    world
        .create_entity()
        .with(sprite_render)
        .with(Ball {
            radius: BALL_RADIUS,
        })
        .with(local_transform)
        .build();
}

fn initialize_doors(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut north_transform = Transform::default();
    let mut south_transform = Transform::default();
    let mut east_transform = Transform::default();
    let mut west_transform = Transform::default();

    let y = WANDERABLE_HEIGHT / 2.0;
    let x: f32 = WANDERABLE_WIDTH / 2.0;
    let z: f32 = 0.0;

    north_transform
        .rotate_2d(90.0_f32.to_radians())
        .set_translation_xyz(x, WANDERABLE_HEIGHT - DOOR_WIDTH * 2.0, z);
    south_transform
        .rotate_2d(90.0_f32.to_radians())
        .set_translation_xyz(x, DOOR_WIDTH * 0.5, z);
    east_transform.set_translation_xyz(WANDERABLE_WIDTH - DOOR_WIDTH * 0.5, y, z);
    west_transform.set_translation_xyz(DOOR_WIDTH * 2.0, y, z);

    let sprite_render = SpriteRender::new(sprite_sheet_handle, 1);

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Door::new(Wall::North))
        .with(north_transform)
        .build();

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Door::new(Wall::South))
        .with(south_transform)
        .build();

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Door::new(Wall::East))
        .with(east_transform)
        .build();

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Door::new(Wall::West))
        .with(west_transform)
        .build();
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(WANDERABLE_WIDTH * 0.5, WANDERABLE_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(WANDERABLE_WIDTH, WANDERABLE_HEIGHT))
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "textures/wanderball_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "textures/wanderball_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
