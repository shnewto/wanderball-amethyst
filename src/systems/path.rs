use amethyst::{
    core::{Hidden, Transform},
    derive::SystemDesc,
    ecs::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

use crate::components::{
    path::{Path, PathTile},
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
pub struct PathTileSystem;

impl<'s> System<'s> for PathTileSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Hidden>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, PathTile>,
        ReadStorage<'s, Videographer>,
        Read<'s, WanderballConfig>,
    );

    fn run(
        &mut self,
        (entities, mut hidden_things, transforms, tiles, videographers, _config): Self::SystemData,
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

        let max_x_val = vx + curr_view_width;
        let min_x_val = vx - curr_view_width;

        let max_y_val = vy + curr_view_height;
        let min_y_max = vy - curr_view_height;

        for (tile, entity) in (&tiles, &entities).join() {
            let x = tile.x;
            let y = tile.y;
            if x > max_x_val || x < min_x_val || y > max_y_val || y < min_y_max {
                let _ = hidden_things.insert(entity, Hidden);
            } else {
                let _ = hidden_things.remove(entity);
            }
        }
    }
}
