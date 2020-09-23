use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    error::Error,
};

use crate::systems::BallSystem;
use crate::systems::PathSystem;
#[derive(Default)]
pub struct WanderballBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for WanderballBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(BallSystem, "ball_system", &["input_system"]);
        builder.add(PathSystem, "path_system", &[]);
        // builder.add(MoveBallsSystem, "ball_system", &[]);
        // builder.add(
        //     BounceSystem,
        //     "collision_system",
        //     &["paddle_system", "ball_system"],
        // );
        // builder.add(
        //     WinnerSystem,
        //     "winner_system",
        //     &["paddle_system", "ball_system"],
        // );
        Ok(())
    }
}