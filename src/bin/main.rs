use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    input::Input,
    prelude::*,
};
use bevy_prng::ChaCha8Rng;
use bevy_rand::prelude::*;
use rand::distributions::{Distribution as _, Uniform};
use rand_distr::Normal;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EntropyPlugin::<ChaCha8Rng>::default())
        .add_systems(Startup, setup_scene)
        .add_systems(Update, move_camera)
        .run();
}

struct Planet {
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
) {
    // Spiral
    // x = r(phi)cos(phi)
    // y = r(phi)sin(phi)
    // logarithmic: r = a*exp(k*phi)
    // hyperbolic spiral: r = a / phi

    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            transform: Transform::from_xyz(-120.0, -120., 60.0)
                .looking_at(Vec3::new(0., 0., 0.), Vec3::Z),
            ..default()
        },
        BloomSettings::default(),
    ));

    let material_emissive1 = materials.add(StandardMaterial {
        emissive: Color::rgb_linear(0., 0., 500.),
        ..default()
    });

    for planet in rand_planet_field(500, &mut rng) {
        spawn_planet(
            &planet,
            material_emissive1.clone(),
            &mut commands,
            &mut meshes,
        );
    }
}

#[derive(Component)]
struct PlanetAnchor;

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

fn spawn_planet(
    planet: &Planet,
    material: Handle<StandardMaterial>,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    let Planet { x, y, z, radius } = planet;

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(
                Mesh::try_from(shape::Icosphere {
                    radius: *radius,
                    subdivisions: 5,
                })
                .unwrap(),
            ),
            material,
            transform: Transform::from_xyz(*x, *y, *z),
            ..default()
        },
        PlanetAnchor,
    ));
}

fn rand_planet_field(n: usize, rng: &mut ResMut<GlobalEntropy<ChaCha8Rng>>) -> Vec<Planet> {
    let rng = rng.as_mut();

    let x_scale = 100.;
    let y_scale = 100.;
    let z_scale = 1.;

    let x_ranger = Uniform::from(-1. * x_scale..1. * x_scale);
    let y_ranger = Normal::new(0., y_scale / 2.).unwrap();
    let z_ranger = Uniform::from(-1. * z_scale..1. * z_scale);
    let radius_ranger = Normal::new(0.3, 0.1).unwrap().map(|val: f32| val.max(0.1));

    let mut planets = Vec::with_capacity(n);

    for _ in 0..n {
        planets.push(Planet {
            x: x_ranger.sample(rng),
            y: y_ranger.sample(rng),
            z: z_ranger.sample(rng),
            radius: radius_ranger.sample(rng),
        });
    }

    planets
}
