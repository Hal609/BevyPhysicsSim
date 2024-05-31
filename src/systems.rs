
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
    AABB::new( Vec3::new(0.0, 0.0, 0.0),  Vec3::new(25.0, 0.01, 25.0))));


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
    time: Res<Time>,
    mut query: Query<(
        Entity, 
        &mut AABB, 
        Option<&Movable>, 
        Option<&Collidable>, 
        Option<&mut Force>,
        Option<&mut Velocity>, 
        Option<&mut Position>,
        Option<&Mass>
    )>,
) {
    let mut combinations = query.iter_combinations_mut();
    while let Some([
        (entity1, mut aabb1, movable1, collidable1, mut force1, velocity1, mut position1, mass1), 
        (entity2, mut aabb2, movable2, collidable2, mut force2, velocity2, mut position2, mass2)
    ]) = combinations.fetch_next() {
        if aabb1.intersects(&aabb2) {
            if movable1.is_some() && movable2.is_some() {
                println!("Collision detected between two Movable entities: {:?} and {:?}", entity1, entity2);
                // Handle the response for two Movable objects
            } else if movable1.is_some() && collidable2.is_some() {
                println!("Collision detected between Movable entity {:?} and Collidable entity {:?}", entity1, entity2);
                // Handle the response for Movable (entity1) and Collidable (entity2) objects
                if let Some(mut velocity) = velocity1 {
                    if let Some(mut position) = position1 {
                        if let Some(mut force) = force1 {
                            if let Some(mut mass) = mass1 {
                                let normal = calculate_collision_normal(&aabb1, &aabb2);
                                force.0 = -2.0 * mass.0 * velocity.0.length() * normal * 1.0/time.delta_seconds() * 0.95;
                                position.0 -= aabb1.overlap_push_in_direction(&aabb2, normal);
                            }
                        }
                    }
                }
            } else if collidable1.is_some() && movable2.is_some() {
                println!("Collision detected between Collidable entity {:?} and Movable entity {:?}", entity1, entity2);
                // Handle the response for Collidable (entity1) and Movable (entity2) objects
                if let Some(mut velocity) = velocity2 {
                    if let Some(mut position) = position2 {
                        if let Some(mut force) = force2 {
                            if let Some(mut mass) = mass2 {
                                let normal = calculate_collision_normal(&aabb1, &aabb2);
                                force.0 = -2.0 * mass.0 * velocity.0.length() * normal * 1.0/time.delta_seconds() * 0.95;
                                position.0 -= aabb2.overlap_push_in_direction(&aabb1, normal);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn calculate_collision_normal(aabb1: &AABB, aabb2: &AABB) -> Vec3 {
    let center1 = (aabb1.min + aabb1.max) * 0.5;
    let center2 = (aabb2.min + aabb2.max) * 0.5;
    let difference = center1 - center2;

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