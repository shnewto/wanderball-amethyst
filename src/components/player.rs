use amethyst::{
    ecs::{Component, VecStorage},
};

#[derive(Default)]
pub struct Player;

impl Component for Player {
    type Storage = VecStorage<Self>;
}