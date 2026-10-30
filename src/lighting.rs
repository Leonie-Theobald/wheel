use crate::camera::OrbitCamera;
use bevy::prelude::*;

#[derive(Clone, Component)]
pub struct Lighting;

pub fn setup(mut commands: Commands, mut ambient: ResMut<GlobalAmbientLight>) {
    ambient.brightness = 300.;

    commands.spawn((
        Lighting,
        DirectionalLight {
            color: Color::WHITE,
            illuminance: 20_000.,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_translation(OrbitCamera::init().get_position())
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
