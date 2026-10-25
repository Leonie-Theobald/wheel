pub(crate) mod all_discs;
mod full_disc;
mod gear;
pub(crate) mod modification;
mod single_disc;
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
        SingleDisc::init_gear(4.),
        SingleDisc::init_gear(5.),
        SingleDisc::init_gear(6.),
        SingleDisc::init_full_disc(
            7.,
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            [10, 11, 12, 13, 14, 15, 16, 1, 2, 3, 4, 5, 6, 7, 8, 9],
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
