use crate::disc::single_disc::{COUNT_NUMBERS, DISC_THICKNESS, RING_HEIGHT, ROTATION_STEP};
use bevy::prelude::*;

#[derive(Clone)]
pub(crate) struct FullDisc {
    radius: f32,
    outer_numbers: [u8; COUNT_NUMBERS],
    inner_numbers: [u8; COUNT_NUMBERS],
}

impl FullDisc {
    pub(crate) fn init(radius: f32, outer_numbers: [u8; 16], inner_numbers: [u8; 16]) -> Self {
        Self {
            radius,
            outer_numbers,
            inner_numbers,
        }
    }

    pub(crate) fn get_radius(&self) -> f32 {
        self.radius
    }

    pub(crate) fn get_numbers_and_positions(&self) -> Vec<(u8, Vec3, Quat)> {
        let mut numbers_and_positions = Vec::new();

        let mut rotational_offset = ROTATION_STEP / 2.0;
        let outer_radius = self.radius - 0.5 * RING_HEIGHT;
        let inner_radius = self.radius - 1.5 * RING_HEIGHT;

        for (outer_number, inner_number) in self.outer_numbers.into_iter().zip(self.inner_numbers) {
            // adjust orientation of number to face disc center
            let rotation = Quat::from_rotation_y(rotational_offset);

            let outer_vertical_offset = outer_radius * rotational_offset.sin();
            let outer_horizontal_offset = outer_radius * rotational_offset.cos();
            let outer_translation = Vec3::new(
                outer_vertical_offset,
                DISC_THICKNESS / 2. + 0.08,
                outer_horizontal_offset,
            );
            numbers_and_positions.push((outer_number, outer_translation, rotation));

            let inner_vertical_offset = inner_radius * rotational_offset.sin();
            let inner_horizontal_offset = inner_radius * rotational_offset.cos();
            let inner_translation = Vec3::new(
                inner_vertical_offset,
                DISC_THICKNESS / 2. + 0.08,
                inner_horizontal_offset,
            );
            numbers_and_positions.push((inner_number, inner_translation, rotation));

            rotational_offset += ROTATION_STEP;
        }

        numbers_and_positions
    }

    pub(crate) fn generate_full_disc_mesh(&self) -> Mesh {
        Mesh::from(Extrusion::new(Circle::new(self.radius), DISC_THICKNESS))
    }
}
