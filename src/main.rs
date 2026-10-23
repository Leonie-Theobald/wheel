mod camera;
mod disc;
mod gear;
mod world;

use crate::disc::all_discs::AllDiscs;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<AllDiscs>()
        .add_systems(
            Startup,
            (world::setup, camera::setup, disc::setup, gear::setup),
        )
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
