pub(crate) mod all_discs;
mod full_disc;
mod gear;
pub(crate) mod modification;
pub(crate) mod single_disc;
pub(crate) mod visualization;

use all_discs::AllDiscs;
use bevy::prelude::*;
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
        SingleDisc::init_gear(4., [10, 6, 13, 3, 3, 6, 10, 10], None),
        SingleDisc::init_gear(
            5.,
            [2, 17, 15, 14, 5, 10, 2, 22],
            Some([6, 10, 4, 1, 5, 5, 4, 8, 6, 3, 1, 6, 10, 6, 10, 2]),
        ),
        SingleDisc::init_gear(
            6.,
            [2, 10, 15, 6, 9, 16, 17, 2],
            Some([8, 24, 8, 3, 6, 15, 22, 6, 1, 1, 11, 27, 14, 5, 5, 7]),
        ),
        SingleDisc::init_full_disc(
            7.,
            [2, 15, 23, 19, 3, 2, 3, 27, 20, 11, 27, 10, 19, 10, 13, 10],
            [22, 9, 5, 10, 5, 1, 24, 2, 10, 9, 7, 3, 12, 24, 10, 9],
        ),
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
        let entity = disc.spawn(
            color,
            height_offset,
            &mut commands,
            &mut meshes,
            &mut materials,
        );
        entities.push(entity);

        height_offset -= DISC_THICKNESS + 0.1;
    }

    all_discs.reset_with(entities);
}
