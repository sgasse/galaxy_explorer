use std::f32::consts::PI;

use rand_distr::{Distribution, Uniform};

pub fn spiral(sample_pos_dist: f32, size: f32) -> Vec<[f32; 3]> {
    let mut rng = rand::thread_rng();
    let distr = Uniform::new_inclusive(0.0_f32, 5. * sample_pos_dist);

    let offset = sample_pos_dist / 2.;

    let mut stars = Vec::with_capacity(10000);

    for p in spiral_arms(sample_pos_dist, size) {
        for _ in 0..10 {
            stars.push([
                p[0] + distr.sample(&mut rng) - offset,
                p[1] + distr.sample(&mut rng) - offset,
                p[2] + distr.sample(&mut rng) - offset,
            ]);
        }
    }

    stars
}

pub fn spiral_arms(sample_pos_dist: f32, size: f32) -> impl Iterator<Item = [f32; 3]> {
    let a = size / 2.;
    let rotation = Rotation::Left;

    const PHI_MAX: f32 = 2. * PI;

    let mut phi = 0.;
    let phi_inc = 0.001 * PI;

    let mut first_arm = vec![];
    let mut second_arm = vec![];

    let mut last_point: Option<(f32, f32)> = None;

    while phi < PHI_MAX {
        phi += phi_inc;
        let r = radius(a, phi);
        let (x, z) = xz(r, phi, &rotation);

        if let Some((x0, z0)) = last_point {
            let dist = f32::sqrt((x - x0).powi(2) + (z - z0).powi(2));

            // Skip the point if it is too close.
            if dist < sample_pos_dist {
                continue;
            }
        }

        last_point = Some((x, z));

        first_arm.push([x, 0., z]);
        let (x2, z2) = xz(r, phi + PI, &rotation);
        second_arm.push([x2, 0., z2]);
    }

    first_arm.into_iter().chain(second_arm.into_iter())
}

enum Rotation {
    Left,
    Right,
}

fn radius(a: f32, phi: f32) -> f32 {
    // From "A New Formula Describing the Scaffold Structure of Spiral Galaxies"
    // https://arxiv.org/pdf/0908.0892

    const B: f32 = 0.63;
    const N: f32 = 4.;

    a / f32::ln(B * f32::tan(phi / (2. * N)))
}

fn xz(r: f32, phi: f32, rotation: &Rotation) -> (f32, f32) {
    match rotation {
        Rotation::Left => (-r * f32::cos(phi), r * f32::sin(phi)),
        Rotation::Right => (r * f32::cos(phi), r * f32::sin(phi)),
    }
}
