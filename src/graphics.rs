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
use std::pin::Pin;
use uuid::Uuid;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderer {
    pub texture_raw: &'static [u8],
    pub id: Uuid
}
impl Renderer {
    pub fn get_id(&self) {

    }
    pub fn new(texture_raw: &'static [u8]) -> Renderer {
        Renderer {
            texture_raw,
            id: Uuid::new_v4()
        }
    }
}
pub struct GraphicsSystem {
    texture_cache: HashMap<&'static [u8], RwopsContainer>,
    window_canvas: WindowCanvas
}

impl GraphicsSystem {
    pub fn new(video_subsystem: &VideoSubsystem) -> GraphicsSystem {
        let window = video_subsystem.window("corrina-engine", 800, 600).build().unwrap();
        GraphicsSystem {
            window_canvas: window.into_canvas().build().unwrap(),
            texture_cache: Default::default()
        }
    }
}

struct RwopsContainer(Pin<Canvas<Surface<'static>>>, Pin<Texture<'static>>);

impl<'a> System<'a> for GraphicsSystem {
    type SystemData = (
        ReadStorage<'a, Renderer>,
        ReadStorage<'a, Position>,
    );
    fn run(&mut self, (renderers, positions): Self::SystemData) {
        self.window_canvas.set_draw_color(Color::RGB(0, 255, 255));
        self.window_canvas.clear();
        for (renderer, position) in (&renderers, &positions).join() {

        }
    }
}