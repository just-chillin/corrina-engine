use std::{cell::RefCell, sync::{Arc, Mutex}, any::Any};
use sdl2::{
    pixels::Color,
    render::{Canvas, RenderTarget, WindowCanvas, SurfaceCanvas, TextureCreator, Texture},
    video::Window,
    VideoSubsystem,
    surface::Surface,
};
use specs::{prelude::*, Component, storage::UnprotectedStorage};
use crate::physics::*;
use sdl2::video::WindowContext;
use sdl2::rect::Rect;
use sdl2::log::Category::Render;
use std::collections::HashMap;
use std::cell::UnsafeCell;
use sdl2::rwops::RWops;
use std::rc::Rc;
use std::borrow::Borrow;
use sdl2::render::TextureQuery;
use sdl2::image::ImageRWops;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderer {
    texture_raw: &'static [u8]
}
impl Renderer {
    pub fn get_texture(&self) {
    }
    pub fn new(texture_raw: &'static [u8]) -> Renderer {
        Renderer { texture_raw }
    }
}
pub struct GraphicsSystem<'b> {
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    texture_cache: HashMap<&'static [u8], Texture<'b>>
}

impl GraphicsSystem<'_> {
    pub fn new(video_subsystem: &VideoSubsystem) -> GraphicsSystem {
        let window = video_subsystem.window("corrina-engine", 800, 600).build().unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();
        GraphicsSystem { canvas, texture_creator, texture_cache: Default::default() }
    }
}

struct RwopsContainer {

}

impl<'a> System<'a> for GraphicsSystem<'_> {
    type SystemData = (
        ReadStorage<'a, Renderer>,
        ReadStorage<'a, Position>,
    );
    fn run(&mut self, (renderers, positions): Self::SystemData) {
        self.canvas.set_draw_color(Color::RGB(0, 255, 255));
        self.canvas.clear();
        for (renderer, position) in (&renderers, &positions).join() {
            let rwops = RWops::from_bytes(renderer.texture_raw).unwrap();
            let surface = rwops.load().unwrap();
            let surface_prt = *surface;
            let texture = self.texture_creator.create_texture_from_surface(surface).unwrap();
            self.canvas.copy(&texture, None, Rect::new(100, 100, 100, 100));
        }
    }
}