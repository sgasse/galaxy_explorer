use argh::FromArgs;
use bevy::{
    prelude::*,
    window::{WindowMode, WindowResolution},
};
use bevy_mod_picking::DefaultPickingPlugins;
use galaxy::{
    galaxy::setup_stars,
    pan_cam::{asset_loaded, center_on_clicked, pan_orbit_camera, spawn_pan_orbit_camera},
    GalaxyParams, StarClickedEvent,
};
use galaxy_math::{cluster::cluster, spiral::spiral};

#[derive(FromArgs, PartialEq, Debug)]
/// Render galaxy
struct Args {
    /// radius of a star
    #[argh(option, default = "5.")]
    star_radius: f32,

    /// world diameter
    #[argh(option, default = "50000.0")]
    world_diameter: f32,

    #[argh(subcommand)]
    cmd: GalaxyType,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum GalaxyType {
    Cluster(ClusterArgs),
    Spiral(SpiralArgs),
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "cluster")]
/// Cluster
struct ClusterArgs {
    /// how many grid points for sampling
    #[argh(option, short = 'g', default = "1000")]
    grid_points: usize,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "spiral")]
/// Spiral
struct SpiralArgs {
    /// approximate distance between sample points
    #[argh(option, default = "1000.")]
    sample_dist: f32,

    /// generate right-turning galaxy
    #[argh(switch)]
    right_turning: bool,
}

fn main() {
    let params = {
        #[cfg(not(target_arch = "wasm32"))]
        let args: Args = argh::from_env();

        #[cfg(target_arch = "wasm32")]
        let args = Args {
            star_radius: 5.,
            world_diameter: 50000.,
            cmd: GalaxyType::Spiral(SpiralArgs {
                sample_dist: 1000.,
                right_turning: false,
            }),
        };

        let star_positions = match args.cmd {
            GalaxyType::Cluster(cluster_args) => {
                cluster(cluster_args.grid_points, args.world_diameter)
            }
            GalaxyType::Spiral(spiral_args) => spiral(
                spiral_args.sample_dist,
                args.world_diameter,
                spiral_args.right_turning,
            ),
        };
        println!("Number of stars: {}", star_positions.len());

        GalaxyParams {
            star_positions,
            star_radius: args.star_radius,
        }
    };

    let window = WindowPlugin {
        primary_window: Some(Window {
            name: Some("Galaxy Viewer".to_string()),
            mode: WindowMode::BorderlessFullscreen,
            resolution: WindowResolution::new(1920., 1080.),
            ..default()
        }),
        ..default()
    };

    App::new()
        .insert_resource(params)
        .add_plugins(DefaultPlugins.set(window))
        .add_plugins(DefaultPickingPlugins)
        .add_event::<StarClickedEvent>()
        .add_systems(Startup, setup_stars)
        .add_systems(Startup, spawn_pan_orbit_camera)
        .add_systems(Update, pan_orbit_camera)
        .add_systems(Update, asset_loaded)
        .add_systems(Update, center_on_clicked)
        .run();
}
