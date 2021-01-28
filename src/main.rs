use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::sync::atomic::Ordering;

use specs::{Dispatcher, DispatcherBuilder, World, WorldExt};

use crate::sdl_sys::{IsRunningVal, SdlSys};
use crate::game::my_game_mode::MyGameMode;

mod sdl_sys;
mod game;
mod components;

extern crate sdl2;

struct ECS<'a> {
    world: World,
    is_sdl_running: &'a IsRunningVal,
    dispatcher: Dispatcher<'a, 'a>
}
impl ECS<'_> {
    fn new() -> Self {
        let mut world = World::new();
        let (sdl_sys, is_sdl_running) = SdlSys::new();
        let mut dispatcher = DispatcherBuilder::new()
            .with_thread_local(sdl_sys)
            .build();
        ECS {world, is_sdl_running, dispatcher}
    }

    fn running(&self) -> bool { self.is_sdl_running.get() }
    fn maintain(&mut self) {
        self.dispatcher.dispatch(&self.world);
        self.world.maintain();
    }
}

fn main() {
    let mut ecs = ECS::new();
    let _game_mode = MyGameMode::new(&mut ecs.world);
    while ecs.running() {
        ecs.maintain()
    }
}
