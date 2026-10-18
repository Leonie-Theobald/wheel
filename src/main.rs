mod camera;
mod disc;
mod setup;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup::setup)
        .add_systems(Update, camera::orbit_camera)
        .run();
}
