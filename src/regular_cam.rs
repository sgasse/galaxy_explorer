use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    input::Input,
    prelude::*,
};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            transform: Transform::from_xyz(-120.0, -120., 60.0)
                .looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
            ..default()
        },
        BloomSettings::default(),
    ));
}

pub fn move_camera(
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

    if keyboard.pressed(KeyCode::T) {
        query_camera.get_single_mut().unwrap().translation.z += 0.1;
    }

    if keyboard.pressed(KeyCode::G) {
        query_camera.get_single_mut().unwrap().translation.z -= 0.1;
    }
}
