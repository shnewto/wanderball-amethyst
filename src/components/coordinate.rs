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
        coord_display_x,
        coord_display_y,
        coord_display_z,
        coord_display_width,
        coord_display_height,
        coord_display_color,
        coord_display_font_size
    ) = {
        let config = &world.read_resource::<WanderballConfig>();
        (
            config.coord_display_x,
            config.coord_display_y,
            config.coord_display_z,
            config.coord_display_width,
            config.coord_display_height,
            config.coord_display_color,
            config.coord_display_font_size,
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
        coord_display_x,
        coord_display_y,
        coord_display_z,
        coord_display_width,
        coord_display_height,
    );

    let coordinates = world
        .create_entity()
        .with(coordinate_transform)
        .with(UiText::new(
            font.clone(),
            format!("({},{})", ball_x.to_string(), ball_y.to_string()),
            coord_display_color,
            coord_display_font_size,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    world.insert(CoordinateText { coordinates });
}
