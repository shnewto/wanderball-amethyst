use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadExpect, ReadStorage, System, SystemData, Write, WriteStorage},
    input::{InputHandler, StringBindings},
    ui::UiText,
};

use crate::components::{
    ball::Ball,
    shapes::circle::Circle,
    shapes::rectangle::point_in_rect,
    wanderdata::{Pedometer, PedometerText},
};

use crate::resources::save::PathSegmentRecord;

#[derive(SystemDesc)]
pub struct BallSystem;

impl<'s> System<'s> for BallSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Ball>,
        ReadStorage<'s, Circle>,
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
            circles,
            mut pedometer,
            pedometer_text,
            mut ui_text,
            path_segments,
            input,
        ): Self::SystemData,
    ) {
        let movement_x = input.axis_value("move_x");
        let movement_y = input.axis_value("move_y");
        for (transform, circle, _) in (&mut transforms, &circles, &balls).join() {
            let left = |segment: &PathSegmentRecord| -> f32 {
                segment.transform.translation().x - (segment.rectangle.width * 0.5) + circle.radius
            };
            let bottom = |segment: &PathSegmentRecord| -> f32 {
                segment.transform.translation().y - (segment.rectangle.height * 0.5) + circle.radius
            };
            let right = |segment: &PathSegmentRecord| -> f32 {
                segment.transform.translation().x + (segment.rectangle.width * 0.5) - circle.radius
            };
            let top = |segment: &PathSegmentRecord| -> f32 {
                segment.transform.translation().y + (segment.rectangle.height * 0.5) - circle.radius
            };
            if let Some(mv_amount) = movement_x {
                if mv_amount.floor() as i32 != 0 {
                    let mut threshold = circle.radius * 2.0;
                    if mv_amount < 0.0 {
                        threshold = -threshold;
                    }
                    let new_x = transform.translation().x + (mv_amount as f32);
                    for segment in &(*path_segments) {
                        if point_in_rect(
                            new_x,
                            transform.translation().y,
                            left(segment),
                            bottom(segment),
                            right(segment),
                            top(segment),
                        ) || point_in_rect(
                            new_x + threshold,
                            transform.translation().y,
                            left(segment),
                            bottom(segment),
                            right(segment),
                            top(segment),
                        ) {
                            transform.set_translation_x(new_x);

                            let point = format!(
                                "{}{}",
                                segment.transform.translation().x,
                                segment.transform.translation().y
                            );

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
                    let mut threshold = circle.radius * 2.0;
                    if mv_amount < 0.0 {
                        threshold = -threshold;
                    }
                    let new_y = transform.translation().y + (mv_amount as f32);
                    for segment in &(*path_segments) {
                        if point_in_rect(
                            transform.translation().x,
                            new_y,
                            left(segment),
                            bottom(segment),
                            right(segment),
                            top(segment),
                        ) || point_in_rect(
                            transform.translation().x,
                            new_y + threshold,
                            left(segment),
                            bottom(segment),
                            right(segment),
                            top(segment),
                        ) {
                            transform.set_translation_y(new_y);

                            let point = format!(
                                "{}{}",
                                segment.transform.translation().x,
                                segment.transform.translation().y
                            );

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
