use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_math::primitives::Sphere;
use bevy_prng::ChaCha8Rng;
use bevy_rand::prelude::*;
use rand::{distributions::Distribution as _, seq::SliceRandom as _};
use rand_distr::Normal;

use crate::WorldParams;

const STAR_COLORS: &[(f32, f32, f32)] = &[
    (470000., 460000., 240000.),
    (438000., 424000., 150000.),
    (500000., 490000., 348000.),
    (500000., 500000., 500000.),
    (92000., 92000., 484000.),
    (190000., 190000., 454000.),
];

pub fn setup_scene(
    params: Res<WorldParams>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
) {
    let star_materials: Vec<_> = STAR_COLORS
        .iter()
        .map(|(r, g, b)| {
            materials.add(StandardMaterial {
                emissive: Color::rgb_linear(*r, *g, *b),
                ..Default::default()
            })
        })
        .collect();

    let material_center = materials.add(StandardMaterial {
        emissive: Color::rgb_linear(500000., 500000., 500000.),
        ..default()
    });

    for star in spiral_arm_field(params.number_of_stars, &mut rng) {
        spawn_star(
            &star,
            star_materials.choose(rng.as_mut()).unwrap().clone(),
            &mut commands,
            &mut meshes,
        );
    }

    spawn_star(
        &Star {
            x: 0.,
            y: 0.,
            z: 0.,
            radius: 1.4,
        },
        material_center.clone(),
        &mut commands,
        &mut meshes,
    );
}

struct Star {
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
}

/// Tags and entity as a star.
#[derive(Component)]
struct StarAnchor;

fn spawn_star(
    star: &Star,
    material: Handle<StandardMaterial>,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    let Star { x, y, z, radius } = star;

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(*radius).mesh().ico(5).unwrap()),
            material,
            transform: Transform::from_xyz(*x, *y, *z),
            ..default()
        },
        StarAnchor,
    ));
}

fn rand_star_field(
    n: usize,
    star: &Star,
    rng: &mut ResMut<GlobalEntropy<ChaCha8Rng>>,
) -> Vec<Star> {
    let rng = rng.as_mut();

    let x_scale = 20.;
    let y_scale = 2.;
    let z_scale = 20.;

    let x_ranger = Normal::new(star.x, x_scale / 2.).unwrap();
    let y_ranger = Normal::new(star.y, y_scale / 2.).unwrap();
    let z_ranger = Normal::new(star.z, z_scale / 2.).unwrap();
    let radius_ranger = Normal::new(0.3, 0.1).unwrap().map(|val: f32| val.max(0.1));

    let mut stars = Vec::with_capacity(n);

    for _ in 0..n {
        stars.push(Star {
            x: x_ranger.sample(rng),
            y: y_ranger.sample(rng),
            z: z_ranger.sample(rng),
            radius: radius_ranger.sample(rng),
        });
    }

    stars
}

fn spiral_arm(n: usize, offset: f32) -> Vec<Star> {
    let mut stars = Vec::with_capacity(n);

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

        stars.push(Star { x, y, z, radius });
        t += step;
        phi = f32::ln(t);
    }

    stars
}

fn spiral_arm_field(n: usize, rng: &mut ResMut<GlobalEntropy<ChaCha8Rng>>) -> Vec<Star> {
    let first_arm = spiral_arm(n, 0.);
    let second_arm = spiral_arm(n, (2. / 3.) * PI);
    let third_arm = spiral_arm(n, (4. / 3.) * PI);

    first_arm
        .into_iter()
        .chain(second_arm)
        .chain(third_arm)
        .flat_map(|star| rand_star_field(6, &star, rng))
        .collect()
}
