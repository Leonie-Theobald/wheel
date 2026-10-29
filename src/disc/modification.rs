use super::all_discs::AllDiscs;
use super::single_disc::SingleDisc;
use crate::sum::AllSums;
use bevy::prelude::*;

pub(crate) fn toggle_active_states(keys: Res<ButtonInput<KeyCode>>, mut discs: ResMut<AllDiscs>) {
    if keys.just_pressed(KeyCode::ArrowUp) {
        discs.select_next_upper_disc();
    }
    if keys.just_pressed(KeyCode::ArrowDown) {
        discs.select_next_lower_disc();
    }
}

pub(crate) fn step_in_rotation(
    keys: Res<ButtonInput<KeyCode>>,
    discs: Res<AllDiscs>,
    mut disc_list: Query<&mut SingleDisc>,
    mut all_sums: ResMut<AllSums>,
) {
    let selected_disc_entity = discs.get_selected_disc_entity();
    let mut selected_disc = disc_list.get_mut(selected_disc_entity).unwrap();

    if keys.just_pressed(KeyCode::ArrowLeft) {
        selected_disc.step_once_counter_clockwise();
        AllSums::calculate_sums(&mut all_sums, &discs, &disc_list);
    } else if keys.just_pressed(KeyCode::ArrowRight) {
        selected_disc.step_once_clockwise();
        AllSums::calculate_sums(&mut all_sums, &discs, &disc_list);
    }
}
