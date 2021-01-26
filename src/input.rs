use specs::{Component, VecStorage, System, ReadStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct InputComponent;

pub struct InputSystem;
impl<'a> System<'a> for InputSystem {
    type SystemData = ReadStorage<'a, InputComponent>;
    fn run(&mut self, _: Self::SystemData) {
    }
}