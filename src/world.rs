use bevy::prelude::*;

pub(crate) fn setup(mut commands: Commands) {
    // Light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
