use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadExpect, ReadStorage, System, SystemData, Write, WriteStorage},
    input::{InputHandler, StringBindings},
    ui::UiText,
};

use crate::components::{
    ball::Ball,
    shapes::{rectangle::{point_in_rect, Point2d}},
    videographer::Videographer,
    wanderdata::{Pedometer, PedometerText},
};

use crate::config::WanderballConfig;
use crate::resources::save::PathSegmentRecord;

#[derive(SystemDesc)]
pub struct BallSystem;

impl<'s> System<'s> for BallSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Ball>,
        Write<'s, Pedometer>,
        ReadExpect<'s, PedometerText>,
        WriteStorage<'s, UiText>,
        Read<'s, Vec<PathSegmentRecord>>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(
        &mut self,
        (
            mut transforms,
            balls,
            mut pedometer,
            pedometer_text,
            mut ui_text,
            path_segments,
            input,
        ): Self::SystemData,
    ) {
        let movement_x = input.axis_value("move_x");
        let movement_y = input.axis_value("move_y");
        for (transform, _) in (&mut transforms, &balls).join() {
            let left = |segment: &PathSegmentRecord| -> f32 {
                segment.transform.translation().x - (segment.rectangle.width * 0.5)
            };
            let bottom = |segment: &PathSegmentRecord| -> f32 {
                segment.transform.translation().y - (segment.rectangle.height * 0.5)
            };
            let right = |segment: &PathSegmentRecord| -> f32 {
                segment.transform.translation().x + (segment.rectangle.width * 0.5)
            };
            let top = |segment: &PathSegmentRecord| -> f32 {
                segment.transform.translation().y + (segment.rectangle.height * 0.5)
            };
            if let Some(mv_amount) = movement_x {
                if mv_amount.floor() as i32 != 0 {
                    let new_x = transform.translation().x + (mv_amount as f32);
                    for segment in &(*path_segments) {
                        if point_in_rect(
                            new_x,
                            transform.translation().y,
                            left(segment),
                            bottom(segment),
                            right(segment),
                            top(segment),
                        ) {
                            transform.set_translation_x(new_x);

                            let point = format!("{}{}", segment.transform.translation().x, segment.transform.translation().y);

                            if !pedometer.visited.contains_key(&point) {
                                pedometer.steps += 1;
                                pedometer.visited.insert(point.clone(), ());
                            }

                            break;
                        }
                    }
                }
            }

            if let Some(mv_amount) = movement_y {
                if mv_amount.floor() as i32 != 0 {
                    let new_y = transform.translation().y + (mv_amount as f32);
                    for segment in &(*path_segments) {
                        if point_in_rect(
                            transform.translation().x,
                            new_y,
                            left(segment),
                            bottom(segment),
                            right(segment),
                            top(segment),
                        ) {
                            transform.set_translation_y(new_y);

                            let point = format!("{}{}", segment.transform.translation().x, segment.transform.translation().y);

                            if !pedometer.visited.contains_key(&point) {
                                pedometer.steps += 1;
                                pedometer.visited.insert(point.clone(), ());
                            }

                            break;
                        }
                    }
                }
            }

            if let Some(text) = ui_text.get_mut(pedometer_text.steps) {
                text.text = format!("(path steps: {})", pedometer.steps.to_string());
            }
        }
    }
}
