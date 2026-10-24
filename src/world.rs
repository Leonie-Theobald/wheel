use crate::camera::OrbitCamera;
use bevy::prelude::*;

#[derive(Clone, Component)]
pub(crate) struct Lighting;

pub(crate) fn setup(mut commands: Commands, mut ambient: ResMut<GlobalAmbientLight>) {
    // general lighting
    ambient.color = Color::srgb(0.8, 0.85, 1.0);
    ambient.brightness = 600.;

    commands.spawn((
        Lighting,
        DirectionalLight {
            color: Color::srgb(1.0, 0.92, 0.8),
            illuminance: 20_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_translation(OrbitCamera::init().get_position())
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
