use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    input::Input,
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_scene)
        .add_systems(Update, move_camera)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        BloomSettings::default(),
    ));

    let material_emissive1 = materials.add(StandardMaterial {
        emissive: Color::rgb_linear(100., 100., 500.),
        ..default()
    });

    let mesh = meshes.add(
        Mesh::try_from(shape::Icosphere {
            radius: 0.8,
            subdivisions: 5,
        })
        .unwrap(),
    );

    commands.spawn((
        PbrBundle {
            mesh: mesh.clone(),
            material: material_emissive1,
            transform: Transform::from_xyz(2.0, 0.0, 2.0),
            ..default()
        },
        Planet,
    ));
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
        query_camera.get_single_mut().unwrap().translation.z += 0.1;
    }

    if keyboard.pressed(KeyCode::D) {
        query_camera.get_single_mut().unwrap().translation.z -= 0.1;
    }
}
