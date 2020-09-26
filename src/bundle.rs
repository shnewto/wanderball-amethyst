use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    error::Error,
};

use crate::systems::{BallSystem, PathSystem, PathTileSystem, VideographerSystem};

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
        builder.add(PathTileSystem, "path_tile_system", &[]);
        builder.add(VideographerSystem, "videographer_system", &[]);
        Ok(())
    }
}
