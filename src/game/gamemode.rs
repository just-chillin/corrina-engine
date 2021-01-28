use specs::World;
use crate::game::player::Player;

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
            player: Player {},
            enemies: vec![Enemy {}, Enemy {}],
        }
    }
}