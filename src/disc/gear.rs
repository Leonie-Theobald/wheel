use crate::disc::single_disc::COUNT_NUMBERS;
use crate::disc::single_disc::DISC_THICKNESS;
use crate::disc::single_disc::RING_HEIGHT;
use crate::disc::single_disc::ROTATION_STEP;
use bevy::prelude::*;

#[derive(Clone, Component)]
pub struct Gear {
    radius_outer: f32,
    outer_numbers: [usize; COUNT_NUMBERS / 2],
    inner_numbers: Option<[usize; COUNT_NUMBERS]>,
}

impl Gear {
    pub fn init(
        radius_outer: f32,
        outer_numbers: [usize; 8],
        inner_numbers: Option<[usize; COUNT_NUMBERS]>,
    ) -> Self {
        Self {
            radius_outer,
            outer_numbers,
            inner_numbers,
        }
    }

    pub fn get_radius(&self) -> f32 {
        self.radius_outer
    }

    pub fn get_outer_numbers(&self) -> [Option<usize>; COUNT_NUMBERS] {
        [
            Some(self.outer_numbers[0]),
            None,
            Some(self.outer_numbers[1]),
            None,
            Some(self.outer_numbers[2]),
            None,
            Some(self.outer_numbers[3]),
            None,
            Some(self.outer_numbers[4]),
            None,
            Some(self.outer_numbers[5]),
            None,
            Some(self.outer_numbers[6]),
            None,
            Some(self.outer_numbers[7]),
            None,
        ]
    }

    pub fn get_inner_numbers(&self) -> Option<[usize; COUNT_NUMBERS]> {
        self.inner_numbers
    }

    // Combines a regular polygon representing the inner part of gear and adding COUNT_NUMBERS teeth
    // around it
    pub fn generate_gear_mesh(&self) -> Mesh {
        // initialize with base
        let inner_gear_extrusion = Extrusion::new(
            RegularPolygon::new(self.radius_outer - RING_HEIGHT, COUNT_NUMBERS as u32),
            DISC_THICKNESS,
        );
        let mut gear = Mesh::from(inner_gear_extrusion);

        // add tooth extrusions
        let mut rotational_offset = 0.;
        let count_teeth = COUNT_NUMBERS / 2;
        for _ in 0..count_teeth {
            let tooth_extrusion = self.generate_tooth_extrusion(rotational_offset);
            gear.merge(&tooth_extrusion)
                .expect("Tooth should be possible to merge to gear mesh");

            rotational_offset += 2. * ROTATION_STEP;
        }

        gear
    }

    // Function draws four points (A, B, C, D) which when connected form one tooth
    // It considers the rotational_offset, e.g. PI would be an offset of half a circle, placing the
    // tooth on opposite side
    //
    // Simplified visualization excluding curvature
    //
    //                B___C         _
    //                |   |          |
    //                |   |          | TEETH_HEIGHT
    //         _   ___|   |___      _|
    //         |      A   D
    //         |
    //         |      ^   ^
    //         |      |   |
    //         |      |   |
    // radius_inner   |   |
    //         |      |   rotation_end
    //         |      |
    //         |      rotational_offset
    //         |
    //         |_     x  Center of gear
    fn generate_tooth_extrusion(&self, rotational_offset: f32) -> Mesh {
        // generate 2D outline
        let rotation_end = rotational_offset + ROTATION_STEP;
        let mut points = Vec::new();

        // point A
        let x = (self.radius_outer - RING_HEIGHT) * rotational_offset.sin();
        let y = (self.radius_outer - RING_HEIGHT) * rotational_offset.cos();
        points.push(Vec2::new(x, y));

        // point B
        let x = self.radius_outer * rotational_offset.sin();
        let y = self.radius_outer * rotational_offset.cos();
        points.push(Vec2::new(x, y));

        // point C
        let x = self.radius_outer * rotation_end.sin();
        let y = self.radius_outer * rotation_end.cos();
        points.push(Vec2::new(x, y));

        // point D
        let x = (self.radius_outer - RING_HEIGHT) * rotation_end.sin();
        let y = (self.radius_outer - RING_HEIGHT) * rotation_end.cos();
        points.push(Vec2::new(x, y));

        // extrude and mesh
        let shape = ConvexPolygon::new(points).expect("Polygon must be convex");
        let extrusion = Extrusion::new(shape, DISC_THICKNESS);

        Mesh::from(extrusion)
    }
}
