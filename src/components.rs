use bevy::prelude::*;

// Marker component to identify the movable cube
#[derive(Component)]
pub struct Movable;

// Marker component to identify collidable objects
#[derive(Component)]
pub struct Collidable;

// Position component
#[derive(Component)]
pub struct Position(pub Vec3);

// Velocity component
#[derive(Component)]
pub struct Velocity(pub Vec3);

// Force component
#[derive(Component)]
pub struct Force(pub Vec3);

// Mass component
#[derive(Component)]
pub struct Mass(pub f32);

// AABB component
#[derive(Component)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
    pub half_extents: Vec3,
}

impl AABB {
    pub fn new(center: Vec3, half_extents: Vec3) -> Self {
        Self {
            min: center - half_extents,
            max: center + half_extents,
            half_extents,
        }
    }

    pub fn update(&mut self, center: Vec3) {
        self.min = center - self.half_extents;
        self.max = center + self.half_extents;
    }

    pub fn intersects(&self, other: &AABB) -> bool {
        self.min.x <= other.max.x && self.max.x >= other.min.x &&
        self.min.y <= other.max.y && self.max.y >= other.min.y &&
        self.min.z <= other.max.z && self.max.z >= other.min.z
    }
}
