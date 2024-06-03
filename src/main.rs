//! A simple 3D scene with light shining over a cube sitting on a plane.

use crate::systems::*;
use bevy::prelude::*;
use components::SphereSpawnTimer;

mod components;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .insert_resource(SphereSpawnTimer {
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
        })
        .add_systems(
            Update,
            (
                (apply_gravity, apply_friction, handle_input, spawn_sphere),
                (handle_movable_collision, check_static_collisions).chain(),
                apply_motion,
            )
                .chain(),
        )
        .run();
}
