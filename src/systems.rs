
use bevy::prelude::*;
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
    Normal(Vec3::new(0.0, 1.0, 0.0)),
    AABB::new( Vec3::new(0.0, 0.0, 0.0),  Vec3::new(25.0, 0.01, 25.0))));


    // Ball
    let initial_position = Vec3::new(0.0, 3.5, 0.0);
    let half_extents = Vec3::new(0.5, 0.5, 0.5); // Assuming the cube is 1x1x1

    commands.spawn((PbrBundle {
        mesh: meshes.add(Sphere::default()),
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

pub fn apply_gravity(mut query: Query<&mut Force, With<Movable>>) {
    for mut force in query.iter_mut() {
        let gravity = Vec3::new(0.0, -9.8, 0.0);
        force.0 += gravity;
    }
}

pub fn apply_friction(mut query: Query<(&mut Force, &Velocity), With<Movable>>) {
    for (mut force, velocity) in query.iter_mut() {
        let friction = -velocity.0;
        force.0 += friction;
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
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        for mut force in query.iter_mut() {
            force.0.z -= 10.0; // Add a force to the cube to a positive value when space is pressed
        }
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        for mut force in query.iter_mut() {
            force.0.z += 10.0; // Add a force to the cube to a positive value when space is pressed
        }
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        for mut force in query.iter_mut() {
            force.0.x += 10.0; // Add a force to the cube to a positive value when space is pressed
        }
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        for mut force in query.iter_mut() {
            force.0.x -= 10.0; // Add a force to the cube to a positive value when space is pressed
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
        Option<&Mass>,
        Option<&Normal>,
    )>,
) {
    let mut combinations = query.iter_combinations_mut();
    while let Some([
        (entity1, aabb1, movable1, collidable1, force1, velocity1, position1, mass1, normal1), 
        (entity2, aabb2, movable2, collidable2, force2, velocity2, position2, mass2, normal2)
    ]) = combinations.fetch_next() {
        if aabb1.intersects(&aabb2) {
            if movable1.is_some() && movable2.is_some() {
                println!("Collision detected between two Movable entities: {:?} and {:?}", entity1, entity2);
                // Handle the response for two Movable objects
            } else if movable1.is_some() && collidable2.is_some() {
                println!("Collision detected between Movable entity {:?} and Collidable entity {:?}", entity1, entity2);
                // Handle the response for Movable (entity1) and Collidable (entity2) objects
                if let Some(velocity) = velocity1 {
                    if let Some(mut position) = position1 {
                        if let Some(mut force) = force1 {
                            if let Some(mass) = mass1 {
                                if let Some(normal) = normal2 {
                                    force.0 += -2.0 * mass.0 * velocity.0.length() * normal.0 * 1.0/time.delta_seconds() * 0.95;
                                    position.0 += aabb1.overlap_push_in_direction(&aabb2, normal.0);
                                }
                            }
                        }
                    }
                }
            } else if collidable1.is_some() && movable2.is_some() {
                println!("Collision detected between Collidable entity {:?} and Movable entity {:?}", entity1, entity2);
                // Handle the response for Collidable (entity1) and Movable (entity2) objects
                if let Some(velocity) = velocity2 {
                    if let Some(mut position) = position2 {
                        if let Some(mut force) = force2 {
                            if let Some(mass) = mass2 {
                                if let Some(normal) = normal1 {
                                    force.0 += -2.0 * mass.0 * velocity.0.dot(normal.0) * normal.0 * 1.0/time.delta_seconds() * 0.95;
                                    position.0 += aabb2.overlap_push_in_direction(&aabb1, normal.0);
                                }
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