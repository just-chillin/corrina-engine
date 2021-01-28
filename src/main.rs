mod sdl_sys;

extern crate sdl2;

use specs::{World, DispatcherBuilder, WorldExt};
use crate::sdl_sys::SdlSys;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::sync::atomic::Ordering;


fn main() {
    let mut world = World::new();
    let (sdl_sys, is_running) = SdlSys::new();
    let mut dispatcher = DispatcherBuilder::new()
        .with_thread_local(sdl_sys)
        .build();
    dispatcher.setup(&mut world);
    while is_running.get() {
        dispatcher.dispatch(&world);
        world.maintain();
    }
}
