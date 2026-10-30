use bevy::prelude::*;

use crate::disc::{
    all_discs::AllDiscs,
    single_disc::{COUNT_NUMBERS, SingleDisc},
};

#[derive(Default, Resource)]
pub(crate) struct AllSums {
    sums: [usize; COUNT_NUMBERS],
}

impl AllSums {
    fn reset_sums(&mut self) {
        self.sums = [0; COUNT_NUMBERS];
    }

    pub(crate) fn calculate_sums(
        &mut self,
        discs: &Res<AllDiscs>,
        disc_list: &Query<&mut SingleDisc>,
    ) {
        self.reset_sums();

        for ring in discs.get_visible_numbers(disc_list) {
            for (index, number) in ring.iter().enumerate() {
                self.sums[index] += number;
            }
        }
    }

    pub(crate) fn get_sums(&self) -> [usize; COUNT_NUMBERS] {
        self.sums
    }

    pub(crate) fn sums_equal(&self) -> bool {
        self.sums.iter().all(|sum| sum == &self.sums[0])
    }
}
