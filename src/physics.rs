use specs::{Component, VecStorage, System, ReadStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct Velocity {
    xv: f32, yv: f32
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Position {
    x: f32, y: f32
}