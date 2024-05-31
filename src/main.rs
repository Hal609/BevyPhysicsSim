//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;
use crate::components::*;
use crate::systems::*;

mod components;
mod systems;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (reset_force,
             (apply_gravity, handle_input), check_collisions,
              apply_motion,
        ).chain())  
        .run();
}