use rand_distr::{Distribution, Uniform};

pub fn cube(grid_points: usize, cube_size: f32) -> Vec<[f32; 3]> {
    let n_per_dim = (grid_points as f32).powf(1. / 3.).round();

    let mut x = -cube_size / 2.;
    let mut y = -cube_size / 2.;
    let mut z = -cube_size / 2.;

    let grid_step = cube_size / n_per_dim;

    let n_per_dim = n_per_dim as usize;

    let mut points = Vec::new();

    for _ in 0..n_per_dim {
        for _ in 0..n_per_dim {
            for _ in 0..n_per_dim {
                points.push([x, y, z]);
                z += grid_step;
            }
            y += grid_step;
            z = -cube_size / 2.;
        }
        x += grid_step;
        y = -cube_size / 2.;
    }

    points
}

pub fn cluster(grid_points: usize, size: f32) -> Vec<[f32; 3]> {
    let n_per_dim = (grid_points as f32).powf(1. / 3.).round();

    let mut x = -size / 2.;
    let mut y = -size / 2.;
    let mut z = -size / 2.;

    let grid_step = size / n_per_dim;

    let n_per_dim = n_per_dim as usize;

    let mut rng = rand::thread_rng();
    let distr = Uniform::new_inclusive(0.0_f32, grid_step);

    let mut points = Vec::new();

    for _ in 0..n_per_dim {
        for _ in 0..n_per_dim {
            for _ in 0..n_per_dim {
                let dist = distance(x, y, z);

                if dist < size / 2. {
                    let n_dist = num_samples(dist, size);
                    for _ in 0..n_dist {
                        points.push([
                            x + distr.sample(&mut rng),
                            y + distr.sample(&mut rng),
                            z + distr.sample(&mut rng),
                        ]);
                    }
                }
                z += grid_step;
            }
            y += grid_step;
            z = -size / 2.;
        }
        x += grid_step;
        y = -size / 2.;
    }

    points
}

fn distance(x: f32, y: f32, z: f32) -> f32 {
    f32::sqrt(x * x + y * y + z * z)
}

fn num_samples(dist: f32, cube_size: f32) -> usize {
    let sigma = cube_size / 9.;

    const A: f32 = 20.;
    (A * f32::exp(-(dist * dist / (2. * sigma * sigma)))).ceil() as usize
}

#[test]
fn test_num_samples() {
    dbg!(num_samples(250., 500.));
}
