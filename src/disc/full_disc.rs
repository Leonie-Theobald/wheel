use crate::disc::single_disc::{COUNT_NUMBERS, DISC_THICKNESS};
use bevy::prelude::*;

#[derive(Clone)]
pub struct FullDisc {
    radius: f32,
    outer_numbers: [usize; COUNT_NUMBERS],
    inner_numbers: [usize; COUNT_NUMBERS],
}

impl FullDisc {
    pub fn init(
        radius: f32,
        outer_numbers: [usize; COUNT_NUMBERS],
        inner_numbers: [usize; COUNT_NUMBERS],
    ) -> Self {
        Self {
            radius,
            outer_numbers,
            inner_numbers,
        }
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    pub fn get_outer_numbers(&self) -> [Option<usize>; COUNT_NUMBERS] {
        self.outer_numbers.map(Some)
    }

    pub fn get_inner_numbers(&self) -> Option<[usize; COUNT_NUMBERS]> {
        Some(self.inner_numbers)
    }

    pub fn generate_full_disc_mesh(&self) -> Mesh {
        Mesh::from(Extrusion::new(Circle::new(self.radius), DISC_THICKNESS))
    }
}
