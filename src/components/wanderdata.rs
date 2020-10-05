use crate::components::ball::Ball;
use crate::config::WanderballConfig;
use amethyst::{
    assets::Loader,
    core::transform::Transform,
    ecs::{Component, Entity, Join, VecStorage, World},
    prelude::*,
    ui::{Anchor, LineMode, TtfFormat, UiText, UiTransform},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Coordinate {
    pub x: f32,
    pub y: f32,
}

impl Component for Coordinate {
    type Storage = VecStorage<Self>;
}

pub struct CoordinateText {
    pub coordinates: Entity,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Pedometer {
    pub steps: i32,
    pub visited: HashMap<String, ()>,
}

impl Component for Pedometer {
    type Storage = VecStorage<Self>;
}

pub struct PedometerText {
    pub steps: Entity,
}

pub fn init_coordinates(world: &mut World) {
    let mut ball_x: f32 = 0.0;
    let mut ball_y: f32 = 0.0;
    {
        let ball_storage = world.read_storage::<Ball>();
        let transform_storage = world.read_storage::<Transform>();

        for (_, transform) in (&ball_storage, &transform_storage).join() {
            ball_x = transform.translation().x;
            ball_y = transform.translation().y;
        }
    }

    let (
        wanderdata_display_x,
        wanderdata_display_y,
        wanderdata_display_z,
        wanderdata_display_width,
        wanderdata_display_height,
        wanderdata_display_color,
        wanderdata_display_font_size,
    ) = {
        let config = &world.read_resource::<WanderballConfig>();
        (
            config.wanderdata_display_x,
            config.wanderdata_display_y,
            config.wanderdata_display_z,
            config.wanderdata_display_width,
            config.wanderdata_display_height,
            config.wanderdata_display_color,
            config.wanderdata_display_font_size,
        )
    };

    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let coordinate_transform = UiTransform::new(
        "coordinates".to_string(),
        Anchor::TopLeft,
        Anchor::TopLeft,
        wanderdata_display_x,
        wanderdata_display_y,
        wanderdata_display_z,
        wanderdata_display_width,
        wanderdata_display_height,
    );

    let coordinates = world
        .create_entity()
        .with(coordinate_transform)
        .with(UiText::new(
            font.clone(),
            format!("({},{})", ball_x.to_string(), ball_y.to_string()),
            wanderdata_display_color,
            wanderdata_display_font_size,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    world.insert(CoordinateText { coordinates });
}

pub fn init_pedometer(world: &mut World) {
    let (
        wanderdata_display_x,
        wanderdata_display_y,
        wanderdata_display_z,
        wanderdata_display_width,
        wanderdata_display_height,
        wanderdata_display_color,
        wanderdata_display_font_size,
    ) = {
        let config = &world.read_resource::<WanderballConfig>();
        (
            config.wanderdata_display_x,
            config.wanderdata_display_y,
            config.wanderdata_display_z,
            config.wanderdata_display_width,
            config.wanderdata_display_height,
            config.wanderdata_display_color,
            config.wanderdata_display_font_size,
        )
    };

    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let pedometer_transform = UiTransform::new(
        "pedometer".to_string(),
        Anchor::TopRight,
        Anchor::TopRight,
        wanderdata_display_x,
        wanderdata_display_y,
        wanderdata_display_z,
        wanderdata_display_width,
        wanderdata_display_height,
    );

    let steps = world
        .create_entity()
        .with(pedometer_transform)
        .with(UiText::new(
            font.clone(),
            format!("(path steps: {})", 0.0),
            wanderdata_display_color,
            wanderdata_display_font_size,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    world.insert(PedometerText { steps });
}
