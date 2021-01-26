use specs::{Component, VecStorage, System, ReadStorage, Join};
use std::{rc::Rc, collections::HashMap};

#[derive(Component)]
#[storage(VecStorage)]
pub struct InputComponent(HashMap<&'static str, fn(magnitude: f32)>);

pub struct InputSystem;
impl<'a> System<'a> for InputSystem {
    type SystemData = ReadStorage<'a, InputComponent>;
    fn run(&mut self, inputs: Self::SystemData) {
        for input in inputs.join() {

        }
    }
}