use bevy::prelude::*;
use bevy_math::primitives::Sphere;
use bevy_mod_picking::prelude::*;

use crate::{GalaxyParams, StarClickedEvent};

const STAR_COLORS: &[Color] = &[
    Color::rgb_linear(470000., 460000., 240000.),
    Color::rgb_linear(438000., 424000., 150000.),
    Color::rgb_linear(500000., 490000., 348000.),
    Color::rgb_linear(500000., 500000., 500000.),
    Color::rgb_linear(92000., 92000., 484000.),
    Color::rgb_linear(190000., 190000., 454000.),
];

pub fn setup_stars(
    params: Res<GalaxyParams>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let star_materials: Vec<_> = STAR_COLORS
        .iter()
        .map(|color| {
            materials.add(StandardMaterial {
                emissive: *color,
                ..Default::default()
            })
        })
        .collect();

    let material_center = materials.add(StandardMaterial {
        emissive: Color::rgb_linear(500000., 500000., 500000.),
        ..default()
    });

    spawn_star(
        &Star {
            x: 0.,
            y: 0.,
            z: 0.,
            radius: 4. * params.star_radius,
        },
        material_center.clone(),
        &mut commands,
        &mut meshes,
    );

    for (position, color) in params
        .star_positions
        .iter()
        .zip(star_materials.iter().cycle())
    {
        spawn_star(
            &Star {
                x: position[0],
                y: position[1],
                z: position[2],
                radius: params.star_radius,
            },
            color.clone(),
            &mut commands,
            &mut meshes,
        );
    }
}

pub struct Star {
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
}

/// Tags and entity as a star.
#[derive(Component)]
pub struct StarAnchor;

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
        On::<Pointer<Click>>::send_event::<StarClickedEvent>(),
    ));
}
