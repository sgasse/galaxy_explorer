//! Pan/orbit camera module.
//!
//! Adopted from:
//! https://bevy-cheatbook.github.io/cookbook/pan-orbit-camera.html
//!
use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    input::{
        mouse::{MouseMotion, MouseWheel},
        Input,
    },
    prelude::*,
    window::Window,
};

/// Tags an entity as capable of panning and orbiting.
#[derive(Component)]
pub struct PanOrbitCamera {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
        }
    }
}

/// Update the `PanOrbitCamera`.
pub fn pan_orbit_camera(
    window: Query<&Window>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform, &Projection)>,
) {
    let window = window.single();

    let orbit_button = MouseButton::Right;
    let pan_button = MouseButton::Middle;

    let mut pan = Vec2::ZERO;
    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;
    let mut orbit_button_changed = false;

    if input_mouse.pressed(orbit_button) {
        for ev in ev_motion.read() {
            rotation_move += ev.delta;
        }
    } else if input_mouse.pressed(pan_button) {
        // Pan only if we're not rotating at the moment.
        for ev in ev_motion.read() {
            pan += ev.delta;
        }
    }
    for ev in ev_scroll.read() {
        scroll += ev.y;
    }
    if input_mouse.just_released(orbit_button) || input_mouse.just_pressed(orbit_button) {
        orbit_button_changed = true;
    }

    for (mut pan_orbit, mut transform, projection) in query.iter_mut() {
        if orbit_button_changed {
            // Only check for upside down when orbiting started or ended this frame.
            // If the camera is "upside" down, panning horizontally would be inverted, so invert the input to make it correct.
            let up = transform.rotation * Vec3::Y;
            pan_orbit.upside_down = up.y <= 0.0;
        }

        let mut any = false;
        if rotation_move.length_squared() > 0.0 {
            any = true;
            let window = get_primary_window_size(window);
            let delta_x = {
                let delta = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
                if pan_orbit.upside_down {
                    -delta
                } else {
                    delta
                }
            };
            let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation; // rotate around global y axis
            transform.rotation = transform.rotation * pitch; // rotate around local x axis
        } else if pan.length_squared() > 0.0 {
            any = true;
            // Make panning distance independent of resolution and FOV.
            let window = get_primary_window_size(window);
            if let Projection::Perspective(projection) = projection {
                pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
            }
            // Translate by local axes.
            let right = transform.rotation * Vec3::X * -pan.x;
            let up = transform.rotation * Vec3::Y * pan.y;
            // Make panning proportional to distance away from focus point.
            let translation = (right + up) * pan_orbit.radius;
            pan_orbit.focus += translation;
        } else if scroll.abs() > 0.0 {
            any = true;
            pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
            // Do not allow zoom to reach zero or you get stuck.
            pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
        }

        if any {
            // Emulate parent/child to make the yaw/y-axis rotation behave like a turntable.
            // parent = x and y rotation
            // child = z-offset
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation =
                pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
        }
    }

    // Consume any remaining events, so they don't pile up if we don't need them.
    // (and also to avoid Bevy warning us about not checking events every frame update)
    ev_motion.clear();
}

fn get_primary_window_size(window: &Window) -> Vec2 {
    Vec2::new(
        window.resolution.width() as f32,
        window.resolution.height() as f32,
    )
}

/// Spawn a camera supporting panning and orbiting.
pub fn spawn_pan_orbit_camera(mut commands: Commands) {
    let translation = Vec3::new(-50.0, 10.0, -50.0);
    let radius = translation.length();

    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            tonemapping: Tonemapping::TonyMcMapface,
            ..Default::default()
        },
        PanOrbitCamera {
            radius,
            ..Default::default()
        },
        BloomSettings::default(),
    ));
}
