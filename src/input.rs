use specs::{Component, VecStorage, System, ReadStorage};
use std::rc::Rc;

#[derive(Component)]
#[storage(VecStorage)]
pub struct InputComponent(dyn RecievesInput);
impl RecievesInput for InputComponent {
    fn forward(&self) {self.0.forward()}
    fn backward(&self) {self.0.backward()}
    fn left(&self) {self.0.left()}
    fn right(&self) {self.0.right()}
}

pub struct InputSystem;
impl<'a> System<'a> for InputSystem {
    type SystemData = ReadStorage<'a, InputComponent>;
    fn run(&mut self, inputs: Self::SystemData) {
        for input in inputs.join() {

        }
    }
}

pub trait RecievesInput {
    fn forward(&self) {}
    fn backward(&self) {}
    fn left(&self) {}
    fn right(&self) {}
}