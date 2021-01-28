use specs::World;
use crate::game::player::Player;
use crate::game::enemy::Enemy;

pub struct MyGameMode {
    player: Player,
    enemies: Vec<Enemy>,
}

impl MyGameMode {
    pub(crate) fn new(world: &mut World) -> Self {
        Self {
            player: Player {},
            enemies: vec![Enemy {}, Enemy {}],
        }
    }
}