use amethyst::{
    assets::Handle,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Component, DenseVecStorage, Join, Read, ReadStorage, System, SystemData, World},
    input::{InputHandler, StringBindings},
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use crate::config::WanderballConfig;
use crate::side::Side;
use crate::systems::ball::Ball;

pub struct Path {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Path {
    fn new(side: Side, width: f32, height: f32) -> Path {
        Path {
            side,
            width,
            height,
        }
    }
}

impl Component for Path {
    type Storage = DenseVecStorage<Self>;
}

pub fn starting_path(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let (wanderable_height, path_height, path_width) = {
        let config = &world.read_resource::<WanderballConfig>();
        (
            config.wanderable_height,
            config.path_height,
            config.path_width,
        )
    };

    let y = wanderable_height * 0.25;
    let x = 0.0;
    let z: f32 = 0.0;

    let mut west_transform = Transform::default();
    west_transform.set_translation_xyz(x, y, z);

    let sprite_render = SpriteRender::new(sprite_sheet_handle, 1);

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Path::new(Side::West, path_width, path_height))
        .with(west_transform)
        .build();
}

#[derive(SystemDesc)]
pub struct PathSystem;

impl<'s> System<'s> for PathSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Ball>,
        ReadStorage<'s, Path>,
        Read<'s, WanderballConfig>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (transforms, balls, paths, config, input): Self::SystemData) {
        let _movement_x = input.axis_value("move_x");
        let _movement_y = input.axis_value("move_y");
        let (_wanderable_height, _wanderable_width) =
            { (config.wanderable_height, config.wanderable_width) };

        for (ball, transform) in (&balls, &transforms).join() {
            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;
            for (path, path_transform) in (&paths, &transforms).join() {
                let path_x = path_transform.translation().x - (path.width * 0.5);
                let path_y = path_transform.translation().y - (path.height * 0.5);

                if point_in_rect(
                    ball_x,
                    ball_y,
                    path_x - ball.radius,
                    path_y - ball.radius,
                    path_x + path.width + ball.radius,
                    path_y + path.height + ball.radius,
                ) {
                    println!("ball on path!");
                }                
            }
        }

        ()
    }
}

fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}