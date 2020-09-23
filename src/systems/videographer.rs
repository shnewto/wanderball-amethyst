use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{
        Component, Entity, Join, NullStorage, Read, ReadStorage, System, SystemData, World,
        WriteStorage,
    },
    prelude::*,
};

use crate::config::WanderballConfig;
use crate::systems::ball::Ball;
use crate::systems::path::Path;
use crate::util::{point_near_rect, Point, Rectangle};

/// The entity that holds the camera and moves it when it needs to
#[derive(Default)]
pub struct Videographer;

impl Component for Videographer {
    type Storage = NullStorage<Self>;
}

pub fn initialize_videographer(world: &mut World) -> Entity {
    let (height, width) = {
        let config = &world.read_resource::<WanderballConfig>();
        (config.wanderable_width, config.wanderable_height)
    };

    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(width * 0.5, height * 0.5, 1.0);

    world
        .create_entity()
        .with(Videographer::default())
        .with(local_transform)
        .build()
}

#[derive(SystemDesc, Default)]
pub struct VideographerSystem;

impl<'s> System<'s> for VideographerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Videographer>,
        ReadStorage<'s, Ball>,
        ReadStorage<'s, Path>,
        Read<'s, WanderballConfig>,
    );

    fn run(&mut self, (mut transforms, videographers, balls, paths, config): Self::SystemData) {
        let mut ball_x = 0.0;
        let mut ball_y = 0.0;

        let mut path_x = 0.0;
        let mut path_y = 0.0;

        // Get the local position of the ball.
        for (_ball, transform) in (&balls, &transforms).join() {
            ball_x = transform.translation().x as f32;
            ball_y = transform.translation().y as f32;
        }

        // Get the local position of the ball.
        for (_path, transform) in (&paths, &transforms).join() {
            path_x = transform.translation().x as f32;
            path_y = transform.translation().y as f32;
        }
        let point = Point {
            x: ball_x,
            y: ball_y,
        };

        let rect = Rectangle {
            left: path_x - config.ball_radius,
            bottom: path_y - config.ball_radius,
            right: path_x + config.path_width + config.ball_radius,
            top: path_y + config.path_height + config.ball_radius,
        };

        if point_near_rect(point, rect, config.near_threshold) {
            // Follow the ball for awhile, while it's on or near a path
            for (_videographer, transform) in (&videographers, &mut transforms).join() {
                transform.set_translation_x(ball_x);
                transform.set_translation_y(ball_y);
            }
        }
    }
}

// impl<'s> System<'s> for PathSystem {
//     type SystemData = (
//         ReadStorage<'s, Transform>,
//         ReadStorage<'s, Ball>,
//         ReadStorage<'s, Path>,
//         WriteStorage<'s, Camera>,
//         Read<'s, WanderballConfig>,
//         Read<'s, InputHandler<StringBindings>>,
//     );

//     fn run(&mut self, (transforms, balls, paths, mut camera, config, input): Self::SystemData) {
//         let _movement_x = input.axis_value("move_x");
//         let _movement_y = input.axis_value("move_y");
//         let (wanderable_height, wanderable_width, screen_shift_velocity) = {
//             (
//                 config.wanderable_height,
//                 config.wanderable_width,
//                 config.screen_shift_velocity,
//             )
//         };

//         for (ball, ball_transform) in (&balls, &transforms).join() {
//             let ball_x = ball_transform.translation().x;
//             let ball_y = ball_transform.translation().y;
//             for (path, path_transform) in (&paths, &transforms).join() {
//                 let path_x = path_transform.translation().x - (path.width * 0.5);
//                 let path_y = path_transform.translation().y - (path.height * 0.5);

//                 if point_in_rect(
//                     ball_x,
//                     ball_y,
//                     path_x - ball.radius,
//                     path_y - ball.radius,
//                     path_x + path.width + ball.radius,
//                     path_y + path.height + ball.radius,
//                 ) {
//                     let y_0 = ball_y <= ball.radius;
//                     let x_0 = ball_x <= ball.radius;
//                     let y_max = ball_y >= wanderable_height - ball.radius;
//                     let x_max = ball_x >= wanderable_width - ball.radius;

//                     if y_0 || x_0 || y_max || x_max {
//                         // println!("next screen condition!");
//                         camera.
//                     }

//                     // if y_0 {
//                     //     transform.prepend_translation_y(wanderable_height - path.height * 0.5);
//                     // } else if x_0 {
//                     //     transform.prepend_translation_x(wanderable_width - path.width * 0.5);
//                     // } else if y_max {
//                     //     transform.prepend_translation_y(path.height * 0.5);
//                     // } else if x_max {
//                     //     transform.prepend_translation_x(path.width * 0.5);
//                     // }
//                 }
//             }
//         }

//         ()
//     }
// }
