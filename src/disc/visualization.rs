use super::{all_discs::AllDiscs, single_disc::SingleDisc};
use crate::sum::{all_sums::AllSums, sum_number::SumNumber, visualization::SumMaterial};
use bevy::prelude::*;
use bevy_fontmesh::TextMesh;

pub(crate) const COLOR_SELECTED: Color = Color::srgb(0.3, 0.3, 0.3);
pub(crate) const COLOR_NOT_SELECTED: Color = Color::srgb(0.8, 0.8, 0.8);

pub(crate) fn color_discs(
    discs: ResMut<AllDiscs>,
    mut disc_list: Query<&MeshMaterial3d<StandardMaterial>, With<SingleDisc>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // only necessary when the selected disc has changed
    if !discs.is_changed() {
        return;
    }

    // reset color optic and show correct rotation of all discs
    for material_handle in &mut disc_list {
        materials.get_mut(&material_handle.0).unwrap().base_color = COLOR_NOT_SELECTED;
    }

    // change color for selected disc
    let selected_disc_entity = discs.get_selected_disc_entity();
    let material_handle = disc_list.get(selected_disc_entity).unwrap();
    materials.get_mut(&material_handle.0).unwrap().base_color = COLOR_SELECTED;
}

pub(crate) fn rotate_discs(
    mut disc_list: Query<(&SingleDisc, &mut Transform), Changed<SingleDisc>>,
) {
    for (disc, mut transform) in &mut disc_list {
        transform.rotation = Quat::from_rotation_y(disc.get_rotation_angle());
    }
}

pub(crate) fn draw_sums(
    all_sums: Res<AllSums>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    sum_material: Res<SumMaterial>,
    mut sum_list: Query<(&SumNumber, &mut TextMesh)>,
) {
    if !all_sums.is_changed() {
        return;
    }

    let color = if all_sums.sums_equal() {
        Color::srgb(0., 1., 0.)
    } else {
        Color::srgb(1., 0., 0.)
    };

    materials.get_mut(&sum_material.0).unwrap().base_color = color;

    for (sum, mut text_mesh) in &mut sum_list {
        text_mesh.text = all_sums.get_sums()[sum.get_index()].to_string();
    }
}
