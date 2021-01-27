use specs::{prelude::*, WriteStorage, ReadStorage};
use sdl2::{
    Sdl,
    VideoSubsystem,
    EventSubsystem,
    image::{InitFlag, Sdl2ImageContext},
    video::{Window}
};
use crate::input::InputComponent;

struct SdlSys {
    sdl: Sdl,
    sdl_video: VideoSubsystem,
    sdl_image: Sdl2ImageContext,
    sdl_evt: EventSubsystem,
    window: Window
}
impl<'specs> System<'specs> for SdlSys {
    type SystemData = (WriteStorage<'specs, InputComponent>);

    fn run(&mut self, (input_cmps): Self::SystemData) {

    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.sdl = sdl2::init().unwrap();
        self.sdl_video = self.sdl.video().unwrap();
        self.sdl_image = sdl2::image::init(InitFlag::all()).unwrap();
        self.sdl_evt = self.sdl.event().unwrap();
        self.window = self.sdl_video.window("corrina-engine", 800, 600).build().unwrap();

    }
}