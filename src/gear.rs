use bevy::prelude::*;

const GEAR_THICKNES: f32 = 1.;
const TEETH_COUNT: u8 = 8;
const TEETH_HEIGHT: f32 = 1.;
// Angle that covers one tooth and one valley
const ROTATION_STEP: f32 = 2. / TEETH_COUNT as f32 * std::f32::consts::PI;

fn generate_points_for_base(radius_inner: f32, rotational_offset: f32) -> Vec<Vec2> {
    let mut points = Vec::new();

    let rotation_start_of_tooth = rotational_offset;
    let rotation_end_of_tooth = rotational_offset + 0.5 * ROTATION_STEP;
    let rotation_end_of_valley = rotational_offset + ROTATION_STEP;

    // point A
    let x = radius_inner * rotation_start_of_tooth.sin();
    let y = radius_inner * rotation_start_of_tooth.cos();
    points.push(Vec2::new(x, y));

    // point D
    let x = radius_inner * rotation_end_of_tooth.sin();
    let y = radius_inner * rotation_end_of_tooth.cos();
    points.push(Vec2::new(x, y));

    points
}

fn generate_points_for_one_tooth(radius_inner: f32, rotational_offset: f32) -> Vec<Vec2> {
    let mut points = Vec::new();

    let rotation_start_of_tooth = rotational_offset;
    let rotation_end_of_tooth = rotational_offset + 0.5 * ROTATION_STEP;
    let rotation_end_of_valley = rotational_offset + ROTATION_STEP;

    // point A
    let x = radius_inner * rotation_start_of_tooth.sin();
    let y = radius_inner * rotation_start_of_tooth.cos();
    points.push(Vec2::new(x, y));

    // point B
    let x = (radius_inner + TEETH_HEIGHT) * rotation_start_of_tooth.sin();
    let y = (radius_inner + TEETH_HEIGHT) * rotation_start_of_tooth.cos();
    points.push(Vec2::new(x, y));

    // point C
    let x = (radius_inner + TEETH_HEIGHT) * rotation_end_of_tooth.sin();
    let y = (radius_inner + TEETH_HEIGHT) * rotation_end_of_tooth.cos();
    points.push(Vec2::new(x, y));

    // point D
    let x = radius_inner * rotation_end_of_tooth.sin();
    let y = radius_inner * rotation_end_of_tooth.cos();
    points.push(Vec2::new(x, y));

    points
}

fn generate_gear_base_mesh(radius_inner: f32) -> Mesh {
    let mut points = Vec::new();
    let mut rotational_offset = 0.;
    for _ in 0..TEETH_COUNT {
        points.extend_from_slice(&generate_points_for_base(radius_inner, rotational_offset));
        rotational_offset += ROTATION_STEP;
    }

    let shape = ConvexPolygon::new(points).expect("Polygon must be convex");
    let extrusion = Extrusion::new(shape, GEAR_THICKNES);

    Mesh::from(extrusion)
}

fn generate_gear_tooth_meshes(radius_inner: f32) -> Vec<Mesh> {
    let mut meshes = Vec::new();

    let mut rotational_offset = 0.;
    for _ in 0..TEETH_COUNT {
        let points = generate_points_for_one_tooth(radius_inner, rotational_offset);
        let shape = ConvexPolygon::new(points).expect("Polygon must be convex");
        let extrusion = Extrusion::new(shape, GEAR_THICKNES);
        let mesh = Mesh::from(extrusion);

        meshes.push(mesh);

        rotational_offset += ROTATION_STEP;
    }

    meshes
}

// Function draws five points (A, B, C, D, E) which when connected form one tooth plus one valley
// It considers the rotational_offset, e.g. PI would be an offset of half a circle, placing the
// points on opposite side
//
// Simplified visualization excluding curvature
//
//              B___C         _
//              |   |          |
//              |   |          | TEETH_HEIGHT
//          _   |   |___      _|
//         |    A   D
//         |
//         |    ^   ^   ^
//         |    |   |   |
//         |    |   |   rotation_end_of_valley
// radius_inner |    |   |
//         |    |   rotation_end_of_toot
//         |    |
//         |    rotation_start_of_tooth
//         |
//         |_   x  Center of gear
fn generate_gear_mesh(radius_inner: f32) -> Mesh {
    let mut base = generate_gear_base_mesh(radius_inner);

    for mesh in generate_gear_tooth_meshes(radius_inner) {
        base.merge(&mesh)
            .expect("Tooth should be possible to merge to gear mesh");
    }

    base
}

pub(crate) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(generate_gear_mesh(4.))),
        MeshMaterial3d(materials.add(StandardMaterial {
            cull_mode: None,
            ..default()
        })),
        Transform::from_xyz(0., 4., 0.),
    ));
}
