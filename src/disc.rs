pub(crate) mod all_discs;
mod full_disc;
mod gear;
pub(crate) mod modification;
mod single_disc;
pub(crate) mod visualization;

use all_discs::AllDiscs;
use bevy::prelude::*;
use bevy_fontmesh::prelude::*;
use single_disc::SingleDisc;
use visualization::{COLOR_NOT_SELECTED, COLOR_SELECTED};

use crate::disc::single_disc::DISC_THICKNESS;

pub(crate) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut all_discs: ResMut<AllDiscs>,
) {
    let discs_definition = [
        SingleDisc::init_gear(4.),
        SingleDisc::init_gear(5.),
        SingleDisc::init_gear(6.),
        SingleDisc::init_full_disc(7.),
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
        // spawn disc
        let entity = commands
            .spawn((
                disc.clone(),
                Mesh3d(meshes.add(disc.generate_mesh())),
                MeshMaterial3d(materials.add(StandardMaterial {
                    cull_mode: None,
                    base_color: color,
                    ..default()
                })),
                Transform::from_xyz(0., height_offset, 0.0),
            ))
            .id();

        // Spawn writing on disc
        commands.spawn((
            TextMesh {
                text: "HELLO DISC".to_string(),
                style: TextMeshStyle {
                    depth: 0.08,
                    anchor: TextAnchor::Center,
                    justify: JustifyText::Center,
                    ..default()
                },
                ..default()
            },
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::BLACK,
                ..default()
            })),
            Transform {
                translation: Vec3::new(0.0, DISC_THICKNESS / 2. + 0.08, 0.0),
                rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
                scale: Vec3::splat(1.),
            },
            ChildOf(entity),
        ));

        entities.push(entity);

        height_offset -= DISC_THICKNESS + 0.1;
    }

    all_discs.reset_with(entities);
}
