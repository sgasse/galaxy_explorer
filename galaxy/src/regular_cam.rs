use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    input::ButtonInput,
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
        BloomSettings::NATURAL,
    ));
}

pub fn move_camera(
    mut query_camera: Query<&mut Transform, With<Camera>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.pressed(KeyCode::KeyW) {
        query_camera.get_single_mut().unwrap().translation.x += 0.1;
    }

    if keyboard.pressed(KeyCode::KeyS) {
        query_camera.get_single_mut().unwrap().translation.x -= 0.1;
    }

    if keyboard.pressed(KeyCode::KeyA) {
        query_camera.get_single_mut().unwrap().translation.y += 0.1;
    }

    if keyboard.pressed(KeyCode::KeyD) {
        query_camera.get_single_mut().unwrap().translation.y -= 0.1;
    }

    if keyboard.pressed(KeyCode::KeyT) {
        query_camera.get_single_mut().unwrap().translation.z += 0.1;
    }

    if keyboard.pressed(KeyCode::KeyG) {
        query_camera.get_single_mut().unwrap().translation.z -= 0.1;
    }
}
