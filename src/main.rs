mod sdl_sys;
mod player;
mod gamemode;

extern crate sdl2;

use specs::{World, DispatcherBuilder, WorldExt, Dispatcher};
use crate::sdl_sys::{SdlSys, IsRunningVal};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::sync::atomic::Ordering;
use crate::gamemode::{GameMode, MyGameMode};

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

fn setup_game<T: GameMode>(mut mode: T, ecs: &mut ECS) {
    mode.setup();
}

fn main() {
    let mut ecs = ECS::new();
    setup_game(MyGameMode, ecs.borrow_mut());
    while ecs.running() {
        ecs.maintain()
    }
}
