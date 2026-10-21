use bevy::prelude::*;

#[derive(Default)]
pub(crate) enum SelectedDisc {
    #[default]
    Top,
    UpperMiddle,
    LowerMiddle,
    Bottom,
}

#[derive(Default, Resource)]
pub(crate) struct AllDiscs {
    entities: Vec<Entity>,
    selected_disc: SelectedDisc,
}

impl AllDiscs {
    pub(crate) fn reset_with(&mut self, entities: Vec<Entity>) {
        self.entities = entities;
        self.selected_disc = SelectedDisc::Top;
    }

    pub(crate) fn get_selected_disc_entity(&self) -> Entity {
        match &self.selected_disc {
            SelectedDisc::Top => self.entities[0],
            SelectedDisc::UpperMiddle => self.entities[1],
            SelectedDisc::LowerMiddle => self.entities[2],
            SelectedDisc::Bottom => self.entities[3],
        }
    }

    /// The discs are vertically stacked \
    /// This function makes the disc above the currently selected disc the newly selected disc \
    /// If the currently selected disc is the top one, it wraps around to the bottom disc
    pub(crate) fn select_next_upper_disc(&mut self) {
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
    pub(crate) fn select_next_lower_disc(&mut self) {
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
}
