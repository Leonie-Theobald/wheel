use crate::camera::{OrbitCamera, TARGET_CAMERA_VIEW};
use crate::disc::{ActiveDisc, Disc};
use bevy::prelude::*;

pub(crate) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Discs
    let discs = [
        Disc::init(5., true),
        Disc::init(10., false),
        Disc::init(15., false),
    ];

    let mut height_offset = 0.0;
    // separately spawn first disc to make it active disc afterwards
    let first_disc = discs.first().expect("Just defined a non-empty disc array");
    let active_disc_entity = commands
        .spawn((
            first_disc.clone(),
            Mesh3d(meshes.add(Cylinder::new(first_disc.get_radius(), 1.))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: first_disc.get_color(),
                ..default()
            })),
            Transform::from_xyz(0., height_offset, 0.)
                // temporarily use a stretched circle to see rotations better
                .with_scale(Vec3::new(1.0, 1.0, 0.3)),
        ))
        .id();
    height_offset -= 1.1;
    commands.insert_resource(ActiveDisc::init(active_disc_entity));

    // go through other discs
    for disc in discs.iter().skip(1) {
        commands.spawn((
            disc.clone(),
            Mesh3d(meshes.add(Cylinder::new(disc.get_radius(), 1.))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: disc.get_color(),
                ..default()
            })),
            Transform::from_xyz(0., height_offset, 0.)
                // temporarily use a stretched circle to see rotations better
                .with_scale(Vec3::new(1.0, 1.0, 0.3)),
        ));

        height_offset -= 1.1;
    }

    // Light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Orbit camera
    let orbit_camera = OrbitCamera::init();
    let position = orbit_camera.get_position();

    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(position).looking_at(TARGET_CAMERA_VIEW, Vec3::Y),
        orbit_camera,
    ));
}
