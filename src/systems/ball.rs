use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadExpect, ReadStorage, System, SystemData, Write, WriteStorage},
    input::{InputHandler, StringBindings},
    ui::UiText,
};

use crate::components::{
    ball::Ball,
    shapes::{circle::Circle, rectangle::point_in_rect},
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
        ReadStorage<'s, Circle>,
        ReadStorage<'s, Videographer>,
        Write<'s, Pedometer>,
        ReadExpect<'s, PedometerText>,
        WriteStorage<'s, UiText>,
        Read<'s, Vec<PathSegmentRecord>>,
        Read<'s, WanderballConfig>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(
        &mut self,
        (
            mut transforms,
            balls,
            circles,
            _videographers,
            mut pedometer,
            pedometer_text,
            mut ui_text,
            path_segments,
            _config,
            input,
        ): Self::SystemData,
    ) {
        let movement_x = input.axis_value("move_x");
        let movement_y = input.axis_value("move_y");
        for (transform, circle, _) in (&mut transforms, &circles, &balls).join() {
            {
                let left = |segment: &PathSegmentRecord, circle: &Circle| -> f32 {
                    (segment.transform.translation().x - (segment.rectangle.width * 0.5))
                        // + circle.radius
                };
                let bottom = |segment: &PathSegmentRecord, circle: &Circle| -> f32 {
                    (segment.transform.translation().y - (segment.rectangle.height * 0.5))
                        // + circle.radius
                };
                let right = |segment: &PathSegmentRecord, circle: &Circle| -> f32 {
                    (segment.transform.translation().x + (segment.rectangle.width * 0.5))
                        // - circle.radius
                };
                let top = |segment: &PathSegmentRecord, circle: &Circle| -> f32 {
                    (segment.transform.translation().y + (segment.rectangle.height * 0.5))
                        // - circle.radius
                };

                if let Some(mv_amount) = movement_x {
                    if mv_amount.floor() as i32 != 0 {
                        let new_x = transform.translation().x + (mv_amount as f32);
                        for segment in &(*path_segments) {
                            if point_in_rect(
                                new_x,
                                transform.translation().y,
                                left(segment, circle),
                                bottom(segment, circle),
                                right(segment, circle),
                                top(segment, circle),
                            ) {
                                log::info!("move {} in the x direction", mv_amount);

                                transform.set_translation_x(new_x);
                                pedometer.steps += mv_amount.abs() as f32;

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
                                left(segment, circle),
                                bottom(segment, circle),
                                right(segment, circle),
                                top(segment, circle),
                            ) {
                                log::info!("move {} in the y direction", mv_amount);

                                transform.set_translation_y(new_y);
                                pedometer.steps += mv_amount.abs() as f32;

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
}
