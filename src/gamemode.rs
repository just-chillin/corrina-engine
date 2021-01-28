pub trait GameMode {
    fn setup(&mut self);
}

pub struct MyGameMode;
impl GameMode for MyGameMode {
    fn setup(&mut self) {
        unimplemented!()
    }
}