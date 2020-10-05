use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, ReadExpect, System, SystemData, Write, WriteStorage},
    ui::UiText,
};

use crate::components::ball::Ball;
use crate::components::wanderdata::{Coordinate, CoordinateText};

#[derive(SystemDesc)]
pub struct CoordinateSystem;

impl<'s> System<'s> for CoordinateSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, Coordinate>,
        ReadExpect<'s, CoordinateText>,
    );

    fn run(
        &mut self,
        (mut balls, mut transforms, mut ui_text, mut coordinates, coordinate_text): Self::SystemData,
    ) {
        for (_, transform) in (&mut balls, &mut transforms).join() {
            coordinates.x = transform.translation().x;
            coordinates.y = transform.translation().y;

            if let Some(text) = ui_text.get_mut(coordinate_text.coordinates) {
                text.text = format!(
                    "({},{})",
                    coordinates.x.to_string(),
                    coordinates.y.to_string()
                );
            }
        }
    }
}
