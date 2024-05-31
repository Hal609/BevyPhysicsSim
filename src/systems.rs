
use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};

use crate::components::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ground plane
    commands.spawn((PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
        material: materials.add(Color::WHITE),
        ..default()
    },
    Collidable,
    AABB::new( Vec3::new(0.0, 0.0, 0.0),  Vec3::new(25.0, 0.0, 25.0))));


    // Cube
    let initial_position = Vec3::new(0.0, 3.5, 0.0);
    let half_extents = Vec3::new(0.5, 0.5, 0.5); // Assuming the cube is 1x1x1

    commands.spawn((PbrBundle {
        mesh: meshes.add(Cuboid::default()),
        material: materials.add(Color::RED),
        ..default()
    },
        Movable,
        Position(initial_position),
        Velocity(Vec3::new(0.0, 0.0, 0.0)),
        Force(Vec3::new(0.0, 0.0, 0.0)),
        Mass(1.0),
        AABB::new(initial_position, half_extents)
    ));
    

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

pub fn apply_gravity(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Position, &mut Velocity, &mut Force, &Mass), With<Movable>>,
) {
    for (mut transform, mut position, mut velocity, mut force, mass) in query.iter_mut() {
        let gravity = Vec3::new(0.0, -9.8, 0.0);
        force.0 += gravity;
    }
}

pub fn apply_motion( time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Position, &mut Velocity, &mut Force, &Mass, &mut AABB), With<Movable>>,
) {
    for (mut transform, mut position, mut velocity, force, mass, mut aabb) in query.iter_mut() {
        velocity.0 += force.0 * mass.0 * time.delta_seconds();
        position.0 += velocity.0 * time.delta_seconds();
        transform.translation = position.0;

        aabb.update(position.0);
        // if position.0.y <= 0.5 {
        //     position.0.y = 0.5;
        //     velocity.0.y = -velocity.0.y * 0.7;
        // }
    }
}

pub fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Force, With<Movable>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for mut force in query.iter_mut() {
            force.0.y += 200.0; // Add a force to the cube to a positive value when space is pressed
        }
    }
}

pub fn reset_force(
    mut query: Query<&mut Force, With<Movable>>,
) {
    for mut force in query.iter_mut() {
        force.0 = Vec3::new(0.0, 0.0, 0.0);
    }
}

pub fn check_collisions(
    mut query: Query<(Entity, &AABB, Option<&Movable>, Option<&Collidable>, Option<&mut Velocity>)>,
) {
    let mut combinations = query.iter_combinations_mut();
    while let Some([(entity1, aabb1, movable1, collidable1, velocity1), (entity2, aabb2, movable2, collidable2, velocity2)]) = combinations.fetch_next() {
        if aabb1.intersects(aabb2) {
            if movable1.is_some() && movable2.is_some() {
                // Both entities are Movable
                println!("Collision detected between two Movable entities: {:?} and {:?}", entity1, entity2);
                // Handle the response for two Movable objects
            } else if movable1.is_some() && collidable2.is_some() {
                // Entity1 is Movable and Entity2 is Collidable
                println!("Collision detected between Movable entity {:?} and Collidable entity {:?}", entity1, entity2);
                // Handle the response for Movable (entity1) and Collidable (entity2) objects
            } else if collidable1.is_some() && movable2.is_some() {
                // Entity2 is Movable and Entity1 is Collidable
                println!("Collision detected between Collidable entity {:?} and Movable entity {:?}", entity1, entity2);
                // Handle the response for Collidable (entity1) and Movable (entity2) objects
                if let Some(mut velocity) = velocity2 {
                    let center1 = (aabb1.min + aabb1.max) * 0.5;
                    let center2 = (aabb2.min + aabb2.max) * 0.5;

                    let normal = (center2 - center1).normalize();
                    velocity.0 = reflect_velocity(velocity.0, normal);
                }
            }
        }
    }
}

fn calculate_collision_normal(aabb1: &AABB, aabb2: &AABB) -> Vec3 {
    // This is a simplified method for calculating the collision normal.
    // In a real scenario, you may need a more complex calculation depending on the exact collision detection method.

    // Calculate the center of each AABB
    let center1 = (aabb1.min + aabb1.max) * 0.5;
    let center2 = (aabb2.min + aabb2.max) * 0.5;

    // Find the difference in position between the centers of the two AABBs
    let difference = center1 - center2;

    // Determine the axis of the collision normal based on the greatest difference
    if difference.x.abs() > difference.y.abs() && difference.x.abs() > difference.z.abs() {
        Vec3::new(difference.x.signum(), 0.0, 0.0)
    } else if difference.y.abs() > difference.x.abs() && difference.y.abs() > difference.z.abs() {
        Vec3::new(0.0, difference.y.signum(), 0.0)
    } else {
        Vec3::new(0.0, 0.0, difference.z.signum())
    }
}

fn reflect_velocity(velocity: Vec3, normal: Vec3) -> Vec3 {
    velocity - 2.0 * velocity.dot(normal) * normal
}
