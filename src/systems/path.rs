use amethyst::{
    core::{Hidden, Transform},
    derive::SystemDesc,
    ecs::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
    renderer::{palette::Srgba, resources::Tint},
};

use crate::components::{
    ball::Ball,
    path::{Path, PathSegment},
    shapes::circle::Circle,
    shapes::rectangle::{point_in_rect, Rectangle},
    videographer::Videographer,
};

use crate::config::WanderballConfig;

#[derive(SystemDesc)]
pub struct PathSystem;

impl<'s> System<'s> for PathSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Path>,
        Read<'s, WanderballConfig>,
    );

    fn run(&mut self, (mut _transforms, _paths, _config): Self::SystemData) {}
}

#[derive(SystemDesc)]
pub struct PathSegmentSystem;

impl<'s> System<'s> for PathSegmentSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Hidden>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, PathSegment>,
        ReadStorage<'s, Rectangle>,
        WriteStorage<'s, Tint>,
        ReadStorage<'s, Ball>,
        ReadStorage<'s, Circle>,
        ReadStorage<'s, Videographer>,
        Read<'s, WanderballConfig>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut hidden_things,
            transforms,
            segments,
            rectangles,
            mut tints,
            balls,
            circles,
            videographers,
            _config,
        ): Self::SystemData,
    ) {
        let mut curr_view_height = 0.0;
        let mut curr_view_width = 0.0;
        let mut vx = 0.0;
        let mut vy = 0.0;

        for (videographer, _) in (&videographers, &transforms).join() {
            curr_view_height = videographer.view_height;
            curr_view_width = videographer.view_width;
            vx = videographer.view_x;
            vy = videographer.view_y;
        }
        let left = |transform: &Transform, _segment: &Rectangle, ball: &Circle| -> f32 {
            transform.translation().x - 1.0 - ball.radius
        };
        let bottom = |transform: &Transform, _segment: &Rectangle, ball: &Circle| -> f32 {
            transform.translation().y - 1.0 - ball.radius
        };
        let right = |transform: &Transform, _segment: &Rectangle, ball: &Circle| -> f32 {
            transform.translation().x + 1.0 + ball.radius
        };
        let top = |transform: &Transform, _segment: &Rectangle, ball: &Circle| -> f32 {
            transform.translation().y + 1.0 + ball.radius
        };

        let max_x_val = vx + curr_view_width;
        let min_x_val = vx - curr_view_width;

        let max_y_val = vy + curr_view_height;
        let min_y_max = vy - curr_view_height;

        for (_, rectangle, tint, entity, transform) in
            (&segments, &rectangles, &mut tints, &entities, &transforms).join()
        {
            for (_ball, circle, ball_transform) in (&balls, &circles, &transforms).join() {
                if point_in_rect(
                    ball_transform.translation().x,
                    ball_transform.translation().y,
                    left(transform, rectangle, circle),
                    bottom(transform, rectangle, circle),
                    right(transform, rectangle, circle),
                    top(transform, rectangle, circle),
                ) {
                    *tint = Tint(Srgba::new(0.95, 0.95, 0.95, 1.0));
                }
            }
            let x = transform.translation().x;
            let y = transform.translation().y;
            if x > max_x_val || x < min_x_val || y > max_y_val || y < min_y_max {
                let _ = hidden_things.insert(entity, Hidden);
            } else {
                let _ = hidden_things.remove(entity);
            }
        }
    }
}
