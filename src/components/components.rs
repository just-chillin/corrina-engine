use specs::{Component, VecStorage};

// /*
// Most useful for abstract behaviors such as movement, inventory or attribute management, and other
// non-physical concepts. Actor Components do not have a transform, meaning they do not have any physical
// location or rotation in the world.
//  */
// pub trait ActorComponent {
// }
//
// /*
// support location-based behaviors that do not require a geometric representation.
// This includes spring arms, cameras, physical forces and constraints (but not physical objects), and even audio.
//  */
// pub trait SceneComponent {
// }
//
// /**
// Scene Components with geometric representation, which is generally used to render visual elements or
// to collide or overlap with physical objects. This includes Static or skeletal meshes, sprites or billboards,
// and particle systems as well as box, capsule, and sphere collision volumes.
// */
// pub trait PrimitiveComponent {}

#[derive(Component)]
#[storage(VecStorage)]
pub struct TransformComponent{x: u32, y: u32, w: u32, h: u32}

#[derive(Component)]
#[storage(VecStorage)]
pub struct SpriteDrawComponent;




