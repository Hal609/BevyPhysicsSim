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

    pub fn overlap_push(&self, other: &AABB) -> Vec3 {
        // Calculate the overlap on each axis
        let x_overlap = if other.min.x < self.max.x && other.max.x > self.min.x {
            if self.max.x - other.min.x < other.max.x - self.min.x {
                self.max.x - other.min.x
            } else {
                other.max.x - self.min.x
            }
        } else {
            0.0
        };

        let y_overlap = if other.min.y < self.max.y && other.max.y > self.min.y {
            if self.max.y - other.min.y < other.max.y - self.min.y {
                self.max.y - other.min.y
            } else {
                other.max.y - self.min.y
            }
        } else {
            0.0
        };

        let z_overlap = if other.min.z < self.max.z && other.max.z > self.min.z {
            if self.max.z - other.min.z < other.max.z - self.min.z {
                self.max.z - other.min.z
            } else {
                other.max.z - self.min.z
            }
        } else {
            0.0
        };

        // Find the minimum overlap axis
        let mut min_overlap = x_overlap;
        let mut translation_axis = Vec3::new(x_overlap, 0.0, 0.0);

        if y_overlap != 0.0 && y_overlap < min_overlap {
            min_overlap = y_overlap;
            translation_axis = Vec3::new(0.0, y_overlap, 0.0);
        }

        if z_overlap != 0.0 && z_overlap < min_overlap {
            min_overlap = z_overlap;
            translation_axis = Vec3::new(0.0, 0.0, z_overlap);
        }

        // Adjust the translation axis direction based on the overlap side
        if translation_axis.x != 0.0 {
            if other.min.x < self.min.x {
                translation_axis.x = -translation_axis.x;
            }
        } else if translation_axis.y != 0.0 {
            if other.min.y < self.min.y {
                translation_axis.y = -translation_axis.y;
            }
        } else if translation_axis.z != 0.0 {
            if other.min.z < self.min.z {
                translation_axis.z = -translation_axis.z;
            }
        }

        translation_axis
    }
}
