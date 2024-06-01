//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;
use components::SphereSpawnTimer;
use crate::systems::*;

mod components;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .insert_resource(SphereSpawnTimer {
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
        })
        .add_systems(Update, (reset_force,
             (apply_gravity, apply_friction, handle_input, spawn_sphere), check_collisions,
              apply_motion,
        ).chain())  
        .run();
}