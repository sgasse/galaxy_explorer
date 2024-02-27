use bevy::prelude::*;
use bevy_prng::ChaCha8Rng;
use bevy_rand::prelude::*;
use galaxy_explorer::{
    galaxy::setup_scene,
    pan_cam::{pan_orbit_camera, spawn_pan_orbit_camera},
    WorldParams,
};

use argh::FromArgs;

#[derive(FromArgs)]
/// Show a galaxy view.
struct GalaxyView {
    /// how many planets to sample
    #[argh(option, short = 'n', default = "20")]
    number_of_planets: usize,
}

fn main() {
    let args: GalaxyView = argh::from_env();

    let params = WorldParams {
        number_of_planets: args.number_of_planets,
    };

    App::new()
        .insert_resource(params)
        .add_plugins(DefaultPlugins)
        .add_plugins(EntropyPlugin::<ChaCha8Rng>::default())
        .add_systems(Startup, setup_scene)
        .add_systems(Startup, spawn_pan_orbit_camera)
        .add_systems(Update, pan_orbit_camera)
        .run();
}
