mod camera;
mod disc;
mod lighting;
mod sum;

use crate::{disc::all_discs::AllDiscs, sum::all_sums::AllSums};
use bevy::prelude::*;
use bevy_fontmesh::FontMeshPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FontMeshPlugin::<StandardMaterial>::default())
        .init_resource::<AllSums>()
        .add_systems(
            Startup,
            (lighting::setup, camera::setup, disc::setup, sum::setup).chain(),
        )
        // .add_systems(Update, camera::orbit_camera)
        .add_systems(
            Update,
            (
                camera::orbit_camera,
                sum::visualization::toggle_visibility,
                disc::modification::toggle_active_states,
                disc::modification::step_in_rotation,
                disc::visualization::color_discs,
                disc::visualization::rotate_discs,
                disc::visualization::draw_sums,
            ),
        )
        .run();
}
