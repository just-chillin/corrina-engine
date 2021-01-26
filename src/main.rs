#[macro_use]
extern crate lazy_static;
extern crate sdl2;

use std::sync::{Arc, Mutex};

use sdl2::{render::Canvas, Sdl, video::Window, VideoSubsystem};
use specs::{prelude::*, world};

use graphics::*;
use input::*;
use physics::*;
use std::borrow::Borrow;

mod graphics;
mod input;
mod physics;
mod libsdl;

fn main() {
    let sdl = sdl2::init().unwrap();
    let sdl_video = sdl.video().unwrap();
    let mut world = World::new();
    world.register::<InputComponent>();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Renderer>();

    let mut dispatcher = DispatcherBuilder::new()
        .with(InputSystem, "InputSystem", &[])
        .with_thread_local(GraphicsSystem::new(&sdl_video))
        .build();
    dispatcher.dispatch(&world);
}
