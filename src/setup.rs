use crate::camera;
use bevy::prelude::*;

pub(crate) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Disc
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(10., 1.))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.6, 0.9))),
        Transform::default(),
    ));

    // Light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Orbit camera
    let orbit_camera = camera::OrbitCamera::init();
    let position = orbit_camera.get_position();

    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(position).looking_at(camera::TARGET_CAMERA_VIEW, Vec3::Y),
        orbit_camera,
    ));
}
