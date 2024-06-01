
use bevy::prelude::*;
use rand::prelude::*;
use crate::{components::*, SphereSpawnTimer};

pub fn spawn_sphere(
    time: Res<Time>,
    mut timer: ResMut<SphereSpawnTimer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Update the timer
    if timer.timer.tick(time.delta()).just_finished() {
        // Random position for the sphere
        let mut rng = rand::thread_rng();
        let x: f32 = rng.gen_range(-10.0..10.0);
        let y: f32 = rng.gen_range(1.0..5.0);
        let z: f32 = rng.gen_range(-10.0..10.0);
        let position = Vec3::new(x, y, z);

        // Spawn the sphere
        let half_extents = Vec3::new(0.25, 0.25, 0.25); // Assuming the sphere's bounding box
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Sphere {
                    radius: 0.25
                }),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(rng.gen(), rng.gen(), rng.gen()),  // Random color
                    metallic: 0.0,
                    perceptual_roughness: 0.2,
                    ..Default::default()
                }),
                transform: Transform::from_translation(position),
                ..Default::default()
            },
            Movable,
            Position(position),
            Velocity(Vec3::new(0.0, 0.0, 0.0)),
            Force(Vec3::new(0.0, 0.0, 0.0)),
            Mass(0.25),
            AABB::new(position, half_extents)
        ));
    }
}


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
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(1.0, 0.0, 0.0),  // Red color
            metallic: 0.0,  // Fully metallic
            perceptual_roughness: 0.2,  // Low roughness for a shiny surface
            ..Default::default()
        }),
        ..default()
    },
        Movable,
        Controllable,
        Position(initial_position),
        Velocity(Vec3::new(0.0, 0.0, 0.0)),
        Force(Vec3::new(0.0, 0.0, 0.0)),
        Mass(1.0),
        AABB::new(initial_position, half_extents)
    ));

    // Ball 2
    commands.spawn((PbrBundle {
        mesh: meshes.add(Sphere::default()),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.0, 0.0, 1.0),  // Red color
            metallic: 0.0,  // Fully metallic
            perceptual_roughness: 0.2,  // Low roughness for a shiny surface
            ..Default::default()
        }),
        ..default()
    },
        Movable,
        Position(Vec3::new(-3.0, 1.0, -2.0)),
        Velocity(Vec3::new(0.0, 0.0, 0.0)),
        Force(Vec3::new(0.0, 0.0, 0.0)),
        Mass(1.0),
        AABB::new(Vec3::new(-3.0, 1.0, -2.0), half_extents)
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

pub fn apply_gravity(mut query: Query<(&mut Force, &Mass), With<Movable>>) {
    for (mut force, mass) in query.iter_mut() {
        let gravity = Vec3::new(0.0, -9.8, 0.0);
        force.0 += gravity * mass.0;
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
        velocity.0 += (force.0 / mass.0) * time.delta_seconds();
        position.0 += velocity.0 * time.delta_seconds();
        transform.translation = position.0;
        aabb.update(position.0);
    }
}

pub fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Force, With<Controllable>>,
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
                handle_movable_collision_response(
                    &time, &aabb1, &aabb2, position1,
                    velocity1, force1, mass1,
                    velocity2, force2, mass2,
                );
            } else if movable1.is_some() && collidable2.is_some() {
                handle_static_collision_response(
                    &time, &aabb1, &aabb2, 
                    velocity1, position1, force1, mass1, normal2
                );
            } else if collidable1.is_some() && movable2.is_some() {
                handle_static_collision_response(
                    &time, &aabb2, &aabb1, 
                    velocity2, position2, force2, mass2, normal1
                );
            }
        }
    }
}

fn handle_movable_collision_response(
    time: &Res<Time>,
    aabb1: &AABB,
    aabb2: &AABB,
    mut position1: Option<Mut<Position>>,
    velocity1: Option<Mut<Velocity>>,
    mut force1: Option<Mut<Force>>,
    mass1: Option<&Mass>,
    velocity2: Option<Mut<Velocity>>,
    mut force2: Option<Mut<Force>>,
    mass2: Option<&Mass>
) {
    if let (Some(mut position1), Some(velocity1), Some(mut force1), Some(mass1),
            Some(velocity2), Some(mut force2), Some(mass2)) =
        (position1, velocity1, force1, mass1, velocity2, force2, mass2) {

        let normal = calculate_collision_normal(&aabb1, &aabb2);
        let vel_in_normal = (velocity2.0 - velocity1.0).dot(normal);
        let impulse = normal * -2.0 * vel_in_normal / ((1.0 / mass1.0) + (1.0 / mass2.0));
        
        force1.0 -= impulse / time.delta_seconds();
        force2.0 += impulse / time.delta_seconds();

        position1.0 += aabb1.overlap_push_in_direction(aabb2, normal);
    }
}


fn handle_static_collision_response(
    time: &Res<Time>,
    aabb1: &AABB,
    aabb2: &AABB,
    velocity: Option<Mut<Velocity>>,
    mut position: Option<Mut<Position>>,
    mut force: Option<Mut<Force>>,
    mass: Option<&Mass>,
    normal: Option<&Normal>,
) {
    if let (Some(mut velocity), Some(mut position), Some(mut force), Some(mass), Some(normal)) = (velocity, position, force, mass, normal) {
        force.0 += -2.0 * mass.0 * velocity.0.dot(normal.0) * normal.0 * 1.0 / time.delta_seconds() * 0.95;
        position.0 += aabb1.overlap_push_in_direction(aabb2, normal.0);
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