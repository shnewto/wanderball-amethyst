use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{
        Component, DenseVecStorage, Entity, Join, ReadStorage, System, SystemData, World,
        WriteStorage,
    },
    prelude::*,
};

use crate::config::WanderballConfig;
use crate::side::Side;
use crate::systems::ball::Ball;
use crate::util::{point_on_edge_of_rect, Point};

/// The entity that holds the camera and moves it when it needs to
#[derive(Default)]
pub struct Videographer {
    pub view_radius: f32,
    pub view_x: f32,
    pub view_y: f32,
}

impl Component for Videographer {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialize_videographer(world: &mut World) -> Entity {
    let view_diameter = {
        let config = &world.read_resource::<WanderballConfig>();
        config.view_diameter
    };

    let videographer = Videographer {
        view_radius: view_diameter * 0.5,
        view_x: view_diameter * 0.5,
        view_y: view_diameter * 0.5,
    };

    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(videographer.view_x, videographer.view_y, 2.0);

    world
        .create_entity()
        .with(videographer)
        .with(local_transform)
        .build()
}

#[derive(SystemDesc, Default)]
pub struct VideographerSystem;

impl<'s> System<'s> for VideographerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Videographer>,
        ReadStorage<'s, Ball>,
    );

    fn run(&mut self, (mut transforms, mut videographers, balls): Self::SystemData) {
        let mut ball_x = 0.0;
        let mut ball_y = 0.0;

        // Get the local position of the ball.
        for (_ball, transform) in (&balls, &transforms).join() {
            ball_x = transform.translation().x as f32;
            ball_y = transform.translation().y as f32;
        }

        let point = Point {
            x: ball_x,
            y: ball_y,
        };

        for (videographer, transform) in (&mut videographers, &mut transforms).join() {
            let rect_center = Point {
                x: videographer.view_x,
                y: videographer.view_y,
            };
            if let Some(side) =
                point_on_edge_of_rect(&point, &rect_center, &(videographer.view_radius))
            {
                let mut new_x = videographer.view_x;
                let mut new_y = videographer.view_x;
                let shift_dist = videographer.view_radius * 2.0;

                match side {
                    Side::Left => {
                        new_x = videographer.view_x - shift_dist;
                    }
                    Side::Bottom => {
                        new_y = videographer.view_y - shift_dist;
                    }
                    Side::Right => {
                        new_x = videographer.view_x + shift_dist;
                    }
                    Side::Top => {
                        new_y = videographer.view_y + shift_dist;
                    }
                }

                transform.set_translation_x(new_x);
                transform.set_translation_y(new_y);
                videographer.view_x = new_x;
                videographer.view_y = new_y;
            }
        }
    }
}
