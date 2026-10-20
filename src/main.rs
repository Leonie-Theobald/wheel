mod camera;
mod disc;
mod setup;

use crate::disc::ActiveDisc;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<ActiveDisc>()
        .add_systems(Startup, setup::setup)
        .add_systems(Update, camera::orbit_camera)
        .add_systems(Update, disc::toggle_active_states)
        .add_systems(Update, disc::step_in_rotation)
        .run();
}
