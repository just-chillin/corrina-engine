use crate::game::ents::{Enemy, Player};
use specs::World;

pub trait GameMode {
    fn new(world: &mut World) -> Self;
}

pub struct MyGameMode {
    player: Player,
    enemies: Vec<Enemy>,
}

impl GameMode for MyGameMode {
    fn new(world: &mut World) -> Self {
        Self {
            player: Player::new(world),
            enemies: vec![Enemy::new(world), Enemy::new(world)]
        }
    }
}