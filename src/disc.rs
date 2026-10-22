pub(crate) mod all_discs;
pub(crate) mod modification;
mod single_disc;
pub(crate) mod visualization;

use all_discs::AllDiscs;
use bevy::prelude::*;
use single_disc::SingleDisc;
use visualization::{COLOR_NOT_SELECTED, COLOR_SELECTED};

pub(crate) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut all_discs: ResMut<AllDiscs>,
    asset_server: Res<AssetServer>,
) {
    let discs_definition = [
        SingleDisc::init(5.),
        SingleDisc::init(10.),
        SingleDisc::init(15.),
        SingleDisc::init(20.),
    ];

    // go through discs
    let mut height_offset = 0.0;
    let mut entities = Vec::new();
    for (index, disc) in discs_definition.iter().enumerate() {
        let color = if index == 0 {
            COLOR_SELECTED
        } else {
            COLOR_NOT_SELECTED
        };
        // spawn wheel
        let entity = commands
            .spawn((
                disc.clone(),
                Mesh3d(meshes.add(Cylinder::new(disc.get_radius(), 1.))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: color,
                    ..default()
                })),
                Transform::from_xyz(0., height_offset, 0.),
                // // temporarily use a stretched circle to see rotations better
                // .with_scale(Vec3::new(1.0, 1.0, 0.3)),
            ))
            .id();

        // spawn text with numbers
        commands.spawn((
            Mesh3d(meshes.add(Circle::new(disc.get_radius()))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgba(1.0, 1.0, 1.0, 0.0),
                base_color_texture: Some(asset_server.load(format!("disc_{}.png", index + 1))),
                alpha_mode: AlphaMode::Blend,
                ..default()
            })),
            Transform::from_xyz(0., 0.501, 0.)
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            ChildOf(entity),
        ));

        entities.push(entity);

        height_offset -= 1.1;
    }

    all_discs.reset_with(entities);
}
