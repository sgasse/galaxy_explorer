use bevy::prelude::*;
use bevy_prng::ChaCha8Rng;
use bevy_rand::prelude::*;
use rand::distributions::Distribution as _;
use rand_distr::Normal;

use crate::WorldParams;

const PI: f32 = 3.141592653589793;

pub fn setup_scene(
    params: Res<WorldParams>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
) {
    let material_emissive1 = materials.add(StandardMaterial {
        emissive: Color::rgb_linear(0., 0., 500.),
        ..default()
    });
    let material_emissive2 = materials.add(StandardMaterial {
        emissive: Color::rgb_linear(500., 0., 0.),
        ..default()
    });

    for planet in spiral_arm_field(params.number_of_planets, &mut rng) {
        spawn_planet(
            &planet,
            material_emissive1.clone(),
            &mut commands,
            &mut meshes,
        );
    }

    spawn_planet(
        &Planet {
            x: 0.,
            y: 0.,
            z: 0.,
            radius: 1.4,
        },
        material_emissive2.clone(),
        &mut commands,
        &mut meshes,
    );
}

struct Planet {
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
}

/// Tags and entity as a planet.
#[derive(Component)]
struct PlanetAnchor;

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

fn rand_planet_field(
    n: usize,
    planet: &Planet,
    rng: &mut ResMut<GlobalEntropy<ChaCha8Rng>>,
) -> Vec<Planet> {
    let rng = rng.as_mut();

    let x_scale = 20.;
    let y_scale = 2.;
    let z_scale = 20.;

    let x_ranger = Normal::new(planet.x, x_scale / 2.).unwrap();
    let y_ranger = Normal::new(planet.y, y_scale / 2.).unwrap();
    let z_ranger = Normal::new(planet.z, z_scale / 2.).unwrap();
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

fn spiral_arm(n: usize, offset: f32) -> Vec<Planet> {
    let mut planets = Vec::with_capacity(n);

    // Spiral
    // x = r(phi)cos(phi)
    // y = r(phi)sin(phi)
    // logarithmic: r = a*exp(k*phi)
    // hyperbolic spiral: r = a / phi

    let begin = 1.6;
    let end = f32::exp(2. * PI);
    let step = (end - begin) / n as f32;

    let mut t = begin;
    let mut phi = f32::ln(t);

    while t <= end {
        let r = 1.5 * f32::exp(phi);
        let x = r * f32::cos(phi + offset);
        let y = 0.;
        let z = r * f32::sin(phi + offset);
        let radius = 0.4;

        planets.push(Planet { x, y, z, radius });
        t += step;
        phi = f32::ln(t);
    }

    planets
}

fn spiral_arm_field(n: usize, rng: &mut ResMut<GlobalEntropy<ChaCha8Rng>>) -> Vec<Planet> {
    let first_arm = spiral_arm(n, 0.);
    let second_arm = spiral_arm(n, (2. / 3.) * PI);
    let third_arm = spiral_arm(n, (4. / 3.) * PI);

    first_arm
        .into_iter()
        .chain(second_arm.into_iter())
        .chain(third_arm.into_iter())
        .flat_map(|planet| rand_planet_field(6, &planet, rng))
        .collect()
}
