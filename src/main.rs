mod camera;
mod disc;
mod world;

use crate::disc::all_discs::AllDiscs;
use bevy::prelude::*;
use bevy_fontmesh::FontMeshPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FontMeshPlugin::<StandardMaterial>::default())
        .init_resource::<AllDiscs>()
        .add_systems(Startup, (world::setup, camera::setup, disc::setup))
        .add_systems(Update, camera::orbit_camera)
        .add_systems(
            Update,
            (
                disc::modification::toggle_active_states,
                disc::modification::step_in_rotation,
                disc::visualization::color_discs,
                disc::visualization::rotate_discs,
            ),
        )
        .run();
}
