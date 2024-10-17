use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

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

#[derive(Debug, Clone, Event)]
pub struct StarClickedEvent {
    pub target: Entity,
}

impl From<ListenerInput<Pointer<Click>>> for StarClickedEvent {
    fn from(value: ListenerInput<Pointer<Click>>) -> Self {
        StarClickedEvent {
            target: value.target,
        }
    }
}
