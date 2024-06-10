use bevy::{
    core_pipeline::bloom::{BloomCompositeMode, BloomSettings},
    prelude::*,
};
use galaxy::pan_cam::{pan_orbit_camera, spawn_pan_orbit_camera};

use argh::FromArgs;

#[derive(FromArgs)]
/// Pick a color
struct ColorPicker {
    /// red
    #[argh(option, short = 'r', default = "10000")]
    red: usize,

    /// green
    #[argh(option, short = 'g', default = "10000")]
    green: usize,

    /// blue
    #[argh(option, short = 'b', default = "10000")]
    blue: usize,
}

#[derive(Resource)]
pub struct WorldParams {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

fn main() {
    let ColorPicker { red, green, blue } = argh::from_env();
    let params = WorldParams { red, green, blue };

    App::new()
        .insert_resource(params)
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_scene)
        .add_systems(Startup, spawn_pan_orbit_camera)
        .add_systems(Update, (pan_orbit_camera, update_bloom_settings))
        .run();
}

fn setup_scene(
    params: Res<WorldParams>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material_emissive1 = materials.add(StandardMaterial {
        emissive: Color::rgb_linear(params.red as f32, params.green as f32, params.blue as f32),
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(20.).mesh().ico(5).unwrap()),
            material: material_emissive1,
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        StarAnchor,
    ));

    commands.spawn(ColorSettings {
        red: 0.,
        blue: 0.,
        green: 0.,
    });

    commands.spawn(
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }),
    );
}

#[derive(Component, Debug)]
struct ColorSettings {
    red: f32,
    green: f32,
    blue: f32,
}

/// Tags and entity as a star.
#[derive(Component)]
struct StarAnchor;

#[allow(clippy::too_many_arguments)]
fn update_bloom_settings(
    mut camera: Query<(Entity, Option<&mut BloomSettings>), With<Camera>>,
    mut text: Query<&mut Text>,
    mut commands: Commands,
    mut color_settings: Query<&mut ColorSettings>,
    mut star: Query<(&mut StarAnchor, &mut Handle<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keycode: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let bloom_settings = camera.single_mut();
    let mut text = text.single_mut();
    let text = &mut text.sections[0].value;

    // Adjust bloom settings as in examples.
    match bloom_settings {
        (entity, Some(mut bloom_settings)) => {
            *text = "BloomSettings (Toggle: Space)\n".to_string();
            text.push_str(&format!("(Q/A) Intensity: {}\n", bloom_settings.intensity));
            text.push_str(&format!(
                "(W/S) Low-frequency boost: {}\n",
                bloom_settings.low_frequency_boost
            ));
            text.push_str(&format!(
                "(E/D) Low-frequency boost curvature: {}\n",
                bloom_settings.low_frequency_boost_curvature
            ));
            text.push_str(&format!(
                "(R/F) High-pass frequency: {}\n",
                bloom_settings.high_pass_frequency
            ));
            text.push_str(&format!(
                "(T/G) Mode: {}\n",
                match bloom_settings.composite_mode {
                    BloomCompositeMode::EnergyConserving => "Energy-conserving",
                    BloomCompositeMode::Additive => "Additive",
                }
            ));
            text.push_str(&format!(
                "(Y/H) Threshold: {}\n",
                bloom_settings.prefilter_settings.threshold
            ));
            text.push_str(&format!(
                "(U/J) Threshold softness: {}\n",
                bloom_settings.prefilter_settings.threshold_softness
            ));

            if keycode.just_pressed(KeyCode::Space) {
                commands.entity(entity).remove::<BloomSettings>();
            }

            let dt = time.delta_seconds();

            if keycode.pressed(KeyCode::KeyA) {
                bloom_settings.intensity -= dt / 10.0;
            }
            if keycode.pressed(KeyCode::KeyQ) {
                bloom_settings.intensity += dt / 10.0;
            }
            bloom_settings.intensity = bloom_settings.intensity.clamp(0.0, 1.0);

            if keycode.pressed(KeyCode::KeyS) {
                bloom_settings.low_frequency_boost -= dt / 10.0;
            }
            if keycode.pressed(KeyCode::KeyW) {
                bloom_settings.low_frequency_boost += dt / 10.0;
            }
            bloom_settings.low_frequency_boost = bloom_settings.low_frequency_boost.clamp(0.0, 1.0);

            if keycode.pressed(KeyCode::KeyD) {
                bloom_settings.low_frequency_boost_curvature -= dt / 10.0;
            }
            if keycode.pressed(KeyCode::KeyE) {
                bloom_settings.low_frequency_boost_curvature += dt / 10.0;
            }
            bloom_settings.low_frequency_boost_curvature =
                bloom_settings.low_frequency_boost_curvature.clamp(0.0, 1.0);

            if keycode.pressed(KeyCode::KeyF) {
                bloom_settings.high_pass_frequency -= dt / 10.0;
            }
            if keycode.pressed(KeyCode::KeyR) {
                bloom_settings.high_pass_frequency += dt / 10.0;
            }
            bloom_settings.high_pass_frequency = bloom_settings.high_pass_frequency.clamp(0.0, 1.0);

            if keycode.pressed(KeyCode::KeyG) {
                bloom_settings.composite_mode = BloomCompositeMode::Additive;
            }
            if keycode.pressed(KeyCode::KeyT) {
                bloom_settings.composite_mode = BloomCompositeMode::EnergyConserving;
            }

            if keycode.pressed(KeyCode::KeyH) {
                bloom_settings.prefilter_settings.threshold -= dt;
            }
            if keycode.pressed(KeyCode::KeyY) {
                bloom_settings.prefilter_settings.threshold += dt;
            }
            bloom_settings.prefilter_settings.threshold =
                bloom_settings.prefilter_settings.threshold.max(0.0);

            if keycode.pressed(KeyCode::KeyJ) {
                bloom_settings.prefilter_settings.threshold_softness -= dt / 10.0;
            }
            if keycode.pressed(KeyCode::KeyU) {
                bloom_settings.prefilter_settings.threshold_softness += dt / 10.0;
            }
            bloom_settings.prefilter_settings.threshold_softness = bloom_settings
                .prefilter_settings
                .threshold_softness
                .clamp(0.0, 1.0);
        }

        (entity, None) => {
            *text = "Bloom: Off (Toggle: Space)".to_string();

            if keycode.just_pressed(KeyCode::Space) {
                commands.entity(entity).insert(BloomSettings::NATURAL);
            }
        }
    }

    // Adjust colors.
    let mut settings = color_settings.single_mut();
    let dt = time.delta_seconds();

    const MAX_COLOR: f32 = 100000.;
    const SMALL_INC_COLOR: f32 = 500.;
    const LARGE_INC_COLOR: f32 = 10000.;

    macro_rules! value_adjuster {
        ($less:expr, $more:expr, $value:expr) => {
            if keycode.just_pressed($less) {
                $value = ($value - SMALL_INC_COLOR * dt).clamp(0., MAX_COLOR);
            }
            if keycode.pressed($less) {
                $value = ($value - LARGE_INC_COLOR * dt).clamp(0., MAX_COLOR);
            }
            if keycode.just_pressed($more) {
                $value = ($value + SMALL_INC_COLOR * dt).clamp(0., MAX_COLOR);
            }
            if keycode.pressed($more) {
                $value = ($value + LARGE_INC_COLOR * dt).clamp(0., MAX_COLOR);
            }
        };
    }

    text.push_str(&format!("(Z/X) red: {}\n", settings.red));
    text.push_str(&format!("(C/V) green: {}\n", settings.green));
    text.push_str(&format!("(B/N) blue: {}\n", settings.blue));
    text.push_str("( M ) Apply color\n");

    value_adjuster!(KeyCode::KeyZ, KeyCode::KeyX, settings.red);
    value_adjuster!(KeyCode::KeyC, KeyCode::KeyV, settings.green);
    value_adjuster!(KeyCode::KeyB, KeyCode::KeyN, settings.blue);

    if keycode.just_pressed(KeyCode::KeyM) {
        let new_material = materials.add(StandardMaterial {
            emissive: Color::rgb_linear(settings.red, settings.green, settings.blue),
            ..default()
        });

        let (_anchor, mut material) = star.single_mut();
        *material = new_material;
    }
}
