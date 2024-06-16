use bevy::ecs::system::Resource;

pub mod galaxy;
pub mod pan_cam;
pub mod regular_cam;
#[cfg(target_arch = "wasm32")]
pub mod wasm;

#[derive(Resource)]
pub struct WorldParams {
    pub number_of_stars: usize,
}
