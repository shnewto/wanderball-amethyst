use amethyst::{
    ecs::{Component, VecStorage},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Player;

impl Component for Player {
    type Storage = VecStorage<Self>;
}