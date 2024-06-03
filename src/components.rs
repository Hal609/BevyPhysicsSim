use bevy::prelude::*;

// PhysicsSphere component
#[derive(Component)]
pub struct PhysicsSphere {
    pub position: Position,
    pub radius: f32,
    pub mass: Mass,
    pub velocity: Velocity,
    pub force: Force,
    pub aabb: AABB,
}

#[derive(Component)]
pub struct StaticCollision {
    pub normal: Normal,
    pub aabb: AABB,
}

#[derive(Resource)]
pub struct SphereSpawnTimer {
    pub timer: Timer,
}

#[derive(Component)]
pub struct Controllable;

// Position component
#[derive(Component)]
pub struct Position(pub Vec3);

// Velocity component
#[derive(Component)]
pub struct Velocity(pub Vec3);

// Normal component
#[derive(Component)]
pub struct Normal(pub Vec3);

// Force component
#[derive(Component)]
pub struct Force(pub Vec3);

// Mass component
#[derive(Component)]
pub struct Mass(pub f32);

// AABB component
#[derive(Component, Clone, Copy, Debug)]
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
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
            && self.min.z <= other.max.z
            && self.max.z >= other.min.z
    }

    pub fn overlap_push_in_direction(&self, other: &AABB, direction: Vec3) -> Vec3 {
        // Normalize the direction vector
        let norm_direction = direction.normalize();

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

        // Project the overlaps onto the direction vector
        let x_projection = x_overlap / norm_direction.x.abs();
        let y_projection = y_overlap / norm_direction.y.abs();
        let z_projection = z_overlap / norm_direction.z.abs();

        // Find the minimum projection
        let mut min_projection = x_projection;

        if y_overlap != 0.0 && y_projection < min_projection {
            min_projection = y_projection;
        }

        if z_overlap != 0.0 && z_projection < min_projection {
            min_projection = z_projection;
        }

        return min_projection * norm_direction;
    }
}
