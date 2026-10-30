pub mod all_sums;
pub mod sum_number;
pub mod visualization;

use crate::AllDiscs;
use crate::disc::single_disc::SingleDisc;
use crate::disc::single_disc::{DISC_THICKNESS, RING_HEIGHT, ROTATION_STEP};
use crate::sum::all_sums::AllSums;
use crate::sum::sum_number::SumNumber;
use crate::sum::visualization::SumMaterial;
use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use bevy_fontmesh::{JustifyText, TextAnchor, TextMesh, TextMeshStyle};

pub fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    discs: Res<AllDiscs>,
    disc_list: Query<&mut SingleDisc>,
    mut all_sums: ResMut<AllSums>,
) {
    // Get sum numbers and their positions
    let mut numbers_and_positions = Vec::new();
    let mut rotational_offset = ROTATION_STEP / 2.0;

    let bottom_disc_entity = discs.get_bottom_disc_entity();
    let radius = disc_list.get(bottom_disc_entity).unwrap().get_radius() + RING_HEIGHT;

    all_sums.calculate_sums(&discs, &disc_list);
    for number in all_sums.get_sums() {
        // adjust orientation of number to face disc center
        let rotation = Quat::from_rotation_y(rotational_offset);

        let translation = Vec3::new(
            radius * rotational_offset.sin(),
            DISC_THICKNESS / 2.0 + 0.04 - 4. * (DISC_THICKNESS + 0.1),
            radius * rotational_offset.cos(),
        );

        numbers_and_positions.push((number, translation, rotation));

        rotational_offset += ROTATION_STEP;
    }

    // spawn the numbers
    let sum_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1., 0., 0.),
        unlit: true,
        ..default()
    });
    commands.insert_resource(SumMaterial(sum_material.clone()));

    for (index, (number, translation, rotation)) in numbers_and_positions.into_iter().enumerate() {
        commands.spawn((
            SumNumber::init(index),
            Visibility::Visible,
            TextMesh {
                text: number.to_string(),
                style: TextMeshStyle {
                    depth: 0.08,
                    anchor: TextAnchor::Center,
                    justify: JustifyText::Center,
                    ..default()
                },
                ..default()
            },
            MeshMaterial3d(sum_material.clone()),
            Transform {
                translation,
                rotation: rotation * Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
                scale: Vec3::splat(0.8),
            },
            NotShadowCaster,
        ));
    }
}
