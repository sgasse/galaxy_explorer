use bevy::prelude::*;
use bevy_math::primitives::Sphere;

use crate::GalaxyParams;

const STAR_COLORS: &[(f32, f32, f32)] = &[
    (470000., 460000., 240000.),
    (438000., 424000., 150000.),
    (500000., 490000., 348000.),
    (500000., 500000., 500000.),
    (92000., 92000., 484000.),
    (190000., 190000., 454000.),
];

pub fn setup_stars(
    params: Res<GalaxyParams>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material_center = materials.add(StandardMaterial {
        emissive: Color::rgb_linear(500000., 500000., 500000.),
        ..default()
    });

    for position in &params.star_positions {
        spawn_star(
            &Star {
                x: position[0],
                y: position[1],
                z: position[2],
                radius: params.star_radius,
            },
            material_center.clone(),
            &mut commands,
            &mut meshes,
        );
    }
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
