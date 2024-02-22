use bevy::ecs::system::Resource;

pub mod galaxy;
pub mod pan_cam;
pub mod regular_cam;

#[derive(Resource)]
pub struct WorldParams {
    pub number_of_planets: usize,
}
