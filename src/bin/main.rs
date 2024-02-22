use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    input::Input,
    prelude::*,
};
use bevy_prng::ChaCha8Rng;
use bevy_rand::prelude::*;
use rand::distributions::{Distribution, Standard, Uniform};
use rand_core::RngCore;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EntropyPlugin::<ChaCha8Rng>::default())
        .add_systems(Startup, setup_scene)
        .add_systems(Update, move_camera)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            transform: Transform::from_xyz(-120.0, -120., 20.0)
                .looking_at(Vec3::new(0., 0., 0.), Vec3::Z),
            ..default()
        },
        BloomSettings::default(),
    ));

    let material_emissive1 = materials.add(StandardMaterial {
        emissive: Color::rgb_linear(0., 0., 500.),
        ..default()
    });

    let x_scale = 100.;
    let y_scale = 100.;
    let z_scale = 1.;

    let x_ranger = Uniform::from(-1. * x_scale..1. * x_scale);
    let y_ranger = Uniform::from(-1. * y_scale..1. * y_scale);
    let z_ranger = Uniform::from(-1. * z_scale..1. * z_scale);
    let radius_ranger = Uniform::from(0.5..1.0);

    let mesh = meshes.add(
        Mesh::try_from(shape::Icosphere {
            radius: 0.8,
            subdivisions: 5,
        })
        .unwrap(),
    );

    for _ in 0..50 {
        let x = x_ranger.sample(rng.as_mut());
        let y = y_ranger.sample(rng.as_mut());
        let z = z_ranger.sample(rng.as_mut());
        let radius = radius_ranger.sample(rng.as_mut());

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(
                    Mesh::try_from(shape::Icosphere {
                        radius,
                        subdivisions: 5,
                    })
                    .unwrap(),
                ),
                material: material_emissive1.clone(),
                transform: Transform::from_xyz(x, y, z),
                ..default()
            },
            Planet,
        ));
    }

    // commands.spawn((
    //     PbrBundle {
    //         mesh: mesh.clone(),
    //         material: material_emissive1,
    //         transform: Transform::from_xyz(2.0, 0.0, 2.0),
    //         ..default()
    //     },
    //     Planet,
    // ));
}

#[derive(Component)]
struct Planet;

fn debug_projection(query_camera: Query<&Projection, With<Camera>>) {
    let projection = query_camera.single();
    match projection {
        Projection::Perspective(persp) => {
            dbg!(persp);
            // we have a perspective projection
        }
        Projection::Orthographic(ortho) => {
            // we have an orthographic projection
            dbg!(ortho);
        }
    }
}

fn move_camera(
    mut query_camera: Query<&mut Transform, With<Camera>>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.pressed(KeyCode::W) {
        query_camera.get_single_mut().unwrap().translation.x += 0.1;
    }

    if keyboard.pressed(KeyCode::S) {
        query_camera.get_single_mut().unwrap().translation.x -= 0.1;
    }

    if keyboard.pressed(KeyCode::A) {
        query_camera.get_single_mut().unwrap().translation.y += 0.1;
    }

    if keyboard.pressed(KeyCode::D) {
        query_camera.get_single_mut().unwrap().translation.y -= 0.1;
    }
}
