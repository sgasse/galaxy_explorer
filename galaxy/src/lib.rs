use bevy::ecs::system::Resource;

pub mod galaxy;
pub mod pan_cam;
pub mod regular_cam;
#[cfg(target_arch = "wasm32")]
pub mod wasm;

#[derive(Resource)]
pub struct GalaxyParams {
    pub star_positions: Vec<[f32; 3]>,
    pub star_radius: f32,
}
