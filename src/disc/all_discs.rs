use crate::disc::single_disc::{COUNT_NUMBERS, SingleDisc};
use bevy::prelude::*;

#[derive(Default)]
pub enum SelectedDisc {
    #[default]
    Top,
    UpperMiddle,
    LowerMiddle,
    Bottom,
}

#[derive(Resource)]
pub struct AllDiscs {
    top_disc: Entity,
    upper_middle_disc: Entity,
    lower_middle_disc: Entity,
    bottom_disc: Entity,
    selected_disc: SelectedDisc,
}

impl AllDiscs {
    pub fn init(
        top_disc: Entity,
        upper_middle_disc: Entity,
        lower_middle_disc: Entity,
        bottom_disc: Entity,
    ) -> Self {
        Self {
            top_disc,
            upper_middle_disc,
            lower_middle_disc,
            bottom_disc,
            selected_disc: SelectedDisc::Top,
        }
    }

    pub fn get_selected_disc_entity(&self) -> Entity {
        match &self.selected_disc {
            SelectedDisc::Top => self.top_disc,
            SelectedDisc::UpperMiddle => self.upper_middle_disc,
            SelectedDisc::LowerMiddle => self.lower_middle_disc,
            SelectedDisc::Bottom => self.bottom_disc,
        }
    }

    pub fn get_bottom_disc_entity(&self) -> Entity {
        self.bottom_disc
    }

    /// The discs are vertically stacked \
    /// This function makes the disc above the currently selected disc the newly selected disc \
    /// If the currently selected disc is the top one, it wraps around to the bottom disc
    pub fn select_next_upper_disc(&mut self) {
        match self.selected_disc {
            SelectedDisc::Top => {
                self.selected_disc = SelectedDisc::Bottom;
            }
            SelectedDisc::UpperMiddle => {
                self.selected_disc = SelectedDisc::Top;
            }
            SelectedDisc::LowerMiddle => {
                self.selected_disc = SelectedDisc::UpperMiddle;
            }
            SelectedDisc::Bottom => {
                self.selected_disc = SelectedDisc::LowerMiddle;
            }
        };
    }

    /// The discs are vertically stacked \
    /// This function makes the disc below the currently selected disc the newly selected disc \
    /// If the currently selected disc is the bottom one, it wraps around to the top disc
    pub fn select_next_lower_disc(&mut self) {
        match self.selected_disc {
            SelectedDisc::Top => {
                self.selected_disc = SelectedDisc::UpperMiddle;
            }
            SelectedDisc::UpperMiddle => {
                self.selected_disc = SelectedDisc::LowerMiddle;
            }
            SelectedDisc::LowerMiddle => {
                self.selected_disc = SelectedDisc::Bottom;
            }
            SelectedDisc::Bottom => {
                self.selected_disc = SelectedDisc::Top;
            }
        };
    }

    pub fn get_visible_numbers(
        &self,
        disc_list: &Query<&mut SingleDisc>,
    ) -> [[usize; COUNT_NUMBERS]; 4] {
        // discs
        let top_disc = disc_list
            .get(self.top_disc)
            .expect("Resource should hold valid entities");
        let upper_middle_disc = disc_list
            .get(self.upper_middle_disc)
            .expect("Resource should hold valid entities");
        let lower_middle_disc = disc_list
            .get(self.lower_middle_disc)
            .expect("Resource should hold valid entities");
        let bottom_disc = disc_list
            .get(self.bottom_disc)
            .expect("Resource should hold valid entities");

        // rotations
        let bottom_disc_rotation = bottom_disc.get_discrete_rotation();
        let lower_middle_disc_rotation = lower_middle_disc.get_discrete_rotation();
        let upper_middle_disc_rotation = upper_middle_disc.get_discrete_rotation();
        let top_disc_rotation = top_disc.get_discrete_rotation();

        // outer ring
        let mut outer_ring = bottom_disc.get_outer_numbers().map(Option::unwrap);
        outer_ring.rotate_right(bottom_disc_rotation);

        // next to outer ring
        let mut complete_ring = bottom_disc
            .get_inner_numbers()
            .expect("Bottom disc has inner numbers");
        complete_ring.rotate_right(bottom_disc_rotation);
        let mut gear_ring = lower_middle_disc.get_outer_numbers();
        gear_ring.rotate_right(lower_middle_disc_rotation);
        let next_to_outer_ring = overlay_complete_ring_with_gear_ring(&complete_ring, &gear_ring);

        // next to inner ring
        let mut complete_ring = lower_middle_disc
            .get_inner_numbers()
            .expect("Lower middle disc has inner numbers");
        complete_ring.rotate_right(lower_middle_disc_rotation);
        let mut gear_ring = upper_middle_disc.get_outer_numbers();
        gear_ring.rotate_right(upper_middle_disc_rotation);
        let next_to_inner_ring = overlay_complete_ring_with_gear_ring(&complete_ring, &gear_ring);

        // inner ring
        let mut complete_ring = upper_middle_disc
            .get_inner_numbers()
            .expect("Upper middle disc has inner numbers");
        complete_ring.rotate_right(upper_middle_disc_rotation);
        let mut gear_ring = top_disc.get_outer_numbers();
        gear_ring.rotate_right(top_disc_rotation);
        let inner_ring = overlay_complete_ring_with_gear_ring(&complete_ring, &gear_ring);

        [
            outer_ring,
            next_to_outer_ring,
            next_to_inner_ring,
            inner_ring,
        ]
    }
}

fn overlay_complete_ring_with_gear_ring(
    complete_ring: &[usize; COUNT_NUMBERS],
    gear_ring: &[Option<usize>; COUNT_NUMBERS],
) -> [usize; COUNT_NUMBERS] {
    complete_ring
        .clone()
        .iter_mut()
        .zip(gear_ring.iter())
        .map(|(dst, src)| src.unwrap_or(*dst))
        .collect::<Vec<_>>()
        .try_into()
        .expect("Size is correct")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlay_ring() {
        let ring = [8; 16];
        let other_ring = [
            None,
            Some(1),
            None,
            Some(1),
            None,
            Some(1),
            None,
            Some(1),
            None,
            Some(1),
            None,
            Some(1),
            None,
            Some(1),
            None,
            Some(1),
        ];

        let result = [8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1];

        assert_eq!(
            result,
            overlay_complete_ring_with_gear_ring(&ring, &other_ring)
        );
    }
}
