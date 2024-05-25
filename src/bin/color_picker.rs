use bevy::prelude::*;
use galaxy_explorer::pan_cam::{pan_orbit_camera, spawn_pan_orbit_camera};

use argh::FromArgs;

#[derive(FromArgs)]
/// Pick a color
struct ColorPicker {
    /// red
    #[argh(option, short = 'r', default = "10")]
    red: usize,

    /// green
    #[argh(option, short = 'g', default = "10")]
    green: usize,

    /// blue
    #[argh(option, short = 'b', default = "10")]
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
        .add_systems(Update, pan_orbit_camera)
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

    commands.spawn(PbrBundle {
        mesh: meshes.add(Sphere::new(1.4).mesh().ico(5).unwrap()),
        material: material_emissive1,
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });
}
