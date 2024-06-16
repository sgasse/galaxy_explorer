use bevy::prelude::*;
use bevy_prng::ChaCha8Rng;
use bevy_rand::prelude::*;
use galaxy::{
    galaxy::setup_scene,
    pan_cam::{pan_orbit_camera, spawn_pan_orbit_camera},
    WorldParams,
};

fn main() {
    #[cfg(target_arch = "wasm32")]
    init_wasm();

    #[cfg(not(target_arch = "wasm32"))]
    let params = {
        use argh::FromArgs;

        #[derive(FromArgs)]
        /// Show a galaxy view.
        struct GalaxyView {
            /// how many stars to sample
            #[argh(option, short = 'n', default = "20")]
            number_of_stars: usize,
        }

        let args: GalaxyView = argh::from_env();
        WorldParams {
            number_of_stars: args.number_of_stars,
        }
    };

    #[cfg(target_arch = "wasm32")]
    let params = WorldParams {
        number_of_stars: 20,
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

#[cfg(target_arch = "wasm32")]
fn init_wasm() {
    use galaxy::wasm::{console, search_params, set_panic_hook};
    set_panic_hook();

    if let Some(params) = search_params() {
        if !params.is_empty() {
            console::log_1(&format!("Search params: {}", params).into());
        }
    }
}
