mod camera;
mod disc;
mod world;

use crate::disc::Discs;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Discs>()
        .add_systems(Startup, (world::setup, camera::setup, disc::setup))
        .add_systems(Update, camera::orbit_camera)
        .add_systems(
            Update,
            (
                disc::toggle_active_states,
                disc::step_in_rotation,
                disc::color_discs,
                disc::rotate_discs,
            ),
        )
        .run();
}
