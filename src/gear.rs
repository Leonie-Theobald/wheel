use bevy::prelude::*;

pub(crate) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let shape = ConvexPolygon::new(vec![
        Vec2::new(0., 0.),
        Vec2::new(0., 4.),
        Vec2::new(2., 6.),
        Vec2::new(4., 4.),
        Vec2::new(4., 0.),
    ])
    .expect("Polygon must be convex");

    let mesh = Extrusion::new(shape, 2.0);

    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(mesh))),
        MeshMaterial3d(materials.add(StandardMaterial {
            cull_mode: None,
            ..default()
        })),
        Transform::from_xyz(0., 4., 0.),
    ));
}
