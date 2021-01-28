use sdl2::{Sdl, VideoSubsystem, EventSubsystem, EventPump};
use sdl2::image::{Sdl2ImageContext, InitFlag};
use std::cell::{RefCell, Ref};
use sdl2::video::Window;
use std::borrow::BorrowMut;
use specs::{WriteStorage, System, Join, World, WorldExt};
use sdl2::event::Event;
use specs::shred::DynamicSystemData;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};

static IS_RUNNING: IsRunningVal = IsRunningVal(AtomicBool::new(false));
pub struct IsRunningVal(AtomicBool);
impl IsRunningVal {
    pub fn get(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
    fn set(&self, val: bool) {
        self.0.store(val, Ordering::Relaxed);
    }
}


pub struct SdlSys {
    sdl: Sdl,
    sdl_video: VideoSubsystem,
    sdl_image: Sdl2ImageContext,
    sdl_evt: EventSubsystem,
    evt_pump: EventPump,
    window: Window,

}

impl SdlSys {
    pub fn new<'a>() -> (SdlSys, &'a IsRunningVal) {
        if IS_RUNNING.0.compare_and_swap(true, true, Ordering::Relaxed) {
            panic!("two instances of sdlsys!")
        }
        let sdl = sdl2::init().unwrap();
        let sdl_video = sdl.video().unwrap();
        let sdl_image = sdl2::image::init(InitFlag::all()).unwrap();
        let sdl_evt = sdl.event().unwrap();
        let evt_pump = sdl.event_pump().unwrap();
        let window = sdl_video.window("corrina-engine", 800, 600).build().unwrap();

        (Self { sdl, sdl_video, sdl_image, sdl_evt, evt_pump, window }, &IS_RUNNING)
    }
}
impl Drop for SdlSys {
    fn drop(&mut self) {
        IS_RUNNING.set(false);
    }
}

impl<'specs> System<'specs> for SdlSys {
    type SystemData = ();

    fn run(&mut self, data: Self::SystemData) {

    }
}