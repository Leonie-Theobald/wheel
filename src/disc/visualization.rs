use super::all_discs::AllDiscs;
use super::single_disc::SingleDisc;
use bevy::prelude::*;

pub(crate) const COLOR_SELECTED: Color = Color::srgb(0.95, 0.35, 0.1);
pub(crate) const COLOR_NOT_SELECTED: Color = Color::srgb(0.2, 0.6, 0.9);

pub(crate) fn color_discs(
    discs: ResMut<AllDiscs>,
    mut disc_list: Query<&MeshMaterial3d<StandardMaterial>>,
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
