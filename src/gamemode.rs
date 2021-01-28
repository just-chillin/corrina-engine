use crate::player::Player;
use crate::ents::{Enemy, Player};

pub trait GameMode {
    fn setup(&mut self);
}

pub struct MyGameMode{
    player: Player,
    enemies: vec![Enemy, Enemy]
}
impl GameMode for MyGameMode {
    fn setup(&mut self) {
        unimplemented!()
    }
}