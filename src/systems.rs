use crate::{components::*, SphereSpawnTimer};
use bevy::prelude::*;
use rand::prelude::*;

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
                mesh: meshes.add(Sphere { radius: 0.25 }),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(rng.gen(), rng.gen(), rng.gen()), // Random color
                    metallic: 0.0,
                    perceptual_roughness: 0.2,
                    ..Default::default()
                }),
                transform: Transform::from_translation(position),
                ..Default::default()
            },
            PhysicsSphere {
                position: Position(position),
                radius: 0.25,
                mass: Mass(0.25),
                velocity: Velocity(Vec3::ZERO),
                force: Force(Vec3::ZERO),
                aabb: AABB::new(position, half_extents),
            },
        ));
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    spawn_ground_plane(&mut commands, &mut meshes, &mut materials);
    spawn_vertical_plane(&mut commands, &mut meshes, &mut materials);
    spawn_ball(&mut commands, &mut meshes, &mut materials);
    spawn_lights(&mut commands);
    spawn_camera(&mut commands);
}

fn spawn_ground_plane(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
            material: materials.add(Color::WHITE),
            ..default()
        },
        StaticCollision {
            normal: Normal(Vec3::new(0.0, 1.0, 0.0)),
            aabb: AABB::new(Vec3::new(0.0, -1.0, 0.0), Vec3::new(25.0, 1.0, 25.0)),
        },
    ));
}

fn spawn_vertical_plane(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(25.0, 25.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                ..Default::default()
            }),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -12.5),
                rotation: Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
                ..Default::default()
            },
            ..default()
        },
        StaticCollision {
            normal: Normal(Vec3::new(0.0, 0.0, 1.0)),
            aabb: AABB::new(Vec3::new(0.0, 12.5, -12.5), Vec3::new(25.0, 25.0, 0.01)),
        },
    ));
}

fn spawn_ball(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let initial_position = Vec3::new(0.0, 3.5, 0.0);
    let half_extents = Vec3::new(0.5, 0.5, 0.5); // Assuming the sphere is 1x1x1

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::default()),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(1.0, 0.0, 0.0), // Red color
                metallic: 0.0,                         // Fully metallic
                perceptual_roughness: 0.2,             // Low roughness for a shiny surface
                ..Default::default()
            }),
            ..default()
        },
        PhysicsSphere {
            position: Position(initial_position),
            radius: 0.5,
            mass: Mass(1.0),
            velocity: Velocity(Vec3::ZERO),
            force: Force(Vec3::ZERO),
            aabb: AABB::new(initial_position, half_extents),
        },
        Controllable,
    ));
}

fn spawn_lights(commands: &mut Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 1000.0,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 10.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn spawn_camera(commands: &mut Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-8.5, 8.5, 19.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

pub fn apply_gravity(mut query: Query<&mut PhysicsSphere>) {
    for mut physics_sphere in query.iter_mut() {
        let gravity = Vec3::new(0.0, -9.8, 0.0);
        let mass = physics_sphere.mass.0;
        physics_sphere.force.0 += gravity * mass;
    }
}

pub fn apply_friction(mut query: Query<&mut PhysicsSphere>) {
    for mut physics_sphere in query.iter_mut() {
        let friction = -physics_sphere.velocity.0;
        physics_sphere.force.0 += friction;
    }
}

pub fn apply_motion(time: Res<Time>, mut query: Query<(&mut Transform, &mut PhysicsSphere)>) {
    for (mut transform, mut sphere) in query.iter_mut() {
        let mass = sphere.mass.0;
        let force = sphere.force.0;
        sphere.velocity.0 += (force / mass) * time.delta_seconds();

        let vel = sphere.velocity.0;
        sphere.position.0 += vel * time.delta_seconds();

        transform.translation = sphere.position.0;
        let pos = sphere.position.0;
        sphere.aabb.update(pos);

        sphere.force.0 = Vec3::ZERO;
    }
}

pub fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut PhysicsSphere, With<Controllable>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for mut physics_sphere in query.iter_mut() {
            physics_sphere.force.0.y += 200.0; // Add a force to the cube to a positive value when space is pressed
        }
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        for mut physics_sphere in query.iter_mut() {
            physics_sphere.force.0.z -= 10.0; // Add a force to the cube to a positive value when space is pressed
        }
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        for mut physics_sphere in query.iter_mut() {
            physics_sphere.force.0.z += 10.0; // Add a force to the cube to a positive value when space is pressed
        }
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        for mut physics_sphere in query.iter_mut() {
            physics_sphere.force.0.x += 10.0; // Add a force to the cube to a positive value when space is pressed
        }
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        for mut physics_sphere in query.iter_mut() {
            physics_sphere.force.0.x -= 10.0; // Add a force to the cube to a positive value when space is pressed
        }
    }
}

pub fn check_static_collisions(
    time: Res<Time>,
    mut physics_query: Query<&mut PhysicsSphere>,
    static_query: Query<&StaticCollision>,
) {
    for mut physics_sphere in physics_query.iter_mut() {
        for static_collision in static_query.iter() {
            if physics_sphere.aabb.intersects(&static_collision.aabb) {
                let aabb1 = physics_sphere.aabb;
                let aabb2 = static_collision.aabb;

                let normal = static_collision.normal.0;
                let mass = physics_sphere.mass.0;
                let vel = physics_sphere.velocity.0;

                physics_sphere.force.0 +=
                    -2.0 * mass * vel.dot(normal) * normal * 1.0 / time.delta_seconds();
                physics_sphere.position.0 += aabb1.overlap_push_in_direction(&aabb2, normal);
            }
        }
    }
}

pub fn handle_movable_collision(time: Res<Time>, mut query: Query<&mut PhysicsSphere>) {
    let mut combinations = query.iter_combinations_mut();

    while let Some([mut sphere1, mut sphere2]) = combinations.fetch_next() {
        if sphere1.aabb.intersects(&sphere2.aabb) {
            if sphere1.radius + sphere2.radius >= (sphere1.position.0 - sphere2.position.0).length()
            {
                let len_dif = sphere1.radius + sphere2.radius
                    - (sphere1.position.0 - sphere2.position.0).length();
                let separation_dir = (sphere1.position.0 - sphere2.position.0).normalize();

                let v1 = sphere1.velocity.0;
                let v2 = sphere2.velocity.0;
                let m1 = sphere1.mass.0;
                let m2 = sphere2.mass.0;

                // let normal = calculate_collision_normal(&sphere1.aabb, &sphere2.aabb);
                let vel_in_normal = (v2 - v1).dot(separation_dir);
                let impulse = separation_dir * -2.0 * vel_in_normal / ((1.0 / m1) + (1.0 / m2));

                sphere1.force.0 -= impulse / time.delta_seconds();
                sphere2.force.0 += impulse / time.delta_seconds();

                sphere1.position.0 += len_dif * separation_dir;
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
