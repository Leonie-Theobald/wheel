use bevy::prelude::*;

const GEAR_THICKNES: f32 = 1.;
const TEETH_COUNT: u8 = 8;
const TEETH_HEIGHT: f32 = 1.;
const ROTATION_STEP: f32 = std::f32::consts::PI / TEETH_COUNT as f32;

#[derive(Clone, Component)]
pub(crate) struct Gear {
    radius_inner: f32,
}

impl Gear {
    pub(crate) fn init(radius_inner: f32) -> Self {
        Self { radius_inner }
    }

    pub(crate) fn get_radius_inner(&self) -> f32 {
        self.radius_inner
    }

    // Combines a regular polygon representing the inner part of gear and adding TEETH_COUNT teeth
    // around it
    pub(crate) fn generate_gear_mesh(&self) -> Mesh {
        // initialize with base
        let inner_gear_extrusion = Extrusion::new(
            RegularPolygon::new(self.radius_inner, (2 * TEETH_COUNT) as u32),
            GEAR_THICKNES,
        );
        let mut gear = Mesh::from(inner_gear_extrusion);

        // add tooth extrusions
        let mut rotational_offset = 0.;
        for _ in 0..TEETH_COUNT {
            let tooth_extrusion = generate_tooth_extrusion(self.radius_inner, rotational_offset);
            gear.merge(&tooth_extrusion)
                .expect("Tooth should be possible to merge to gear mesh");

            rotational_offset += 2. * ROTATION_STEP; // steps for one tooth and one valley
        }

        gear
    }
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
fn generate_tooth_extrusion(radius_inner: f32, rotational_offset: f32) -> Mesh {
    // generate 2D outline
    let rotation_end = rotational_offset + ROTATION_STEP;
    let mut points = Vec::new();

    // point A
    let x = radius_inner * rotational_offset.sin();
    let y = radius_inner * rotational_offset.cos();
    points.push(Vec2::new(x, y));

    // point B
    let x = (radius_inner + TEETH_HEIGHT) * rotational_offset.sin();
    let y = (radius_inner + TEETH_HEIGHT) * rotational_offset.cos();
    points.push(Vec2::new(x, y));

    // point C
    let x = (radius_inner + TEETH_HEIGHT) * rotation_end.sin();
    let y = (radius_inner + TEETH_HEIGHT) * rotation_end.cos();
    points.push(Vec2::new(x, y));

    // point D
    let x = radius_inner * rotation_end.sin();
    let y = radius_inner * rotation_end.cos();
    points.push(Vec2::new(x, y));

    // extrude and mesh
    let shape = ConvexPolygon::new(points).expect("Polygon must be convex");
    let extrusion = Extrusion::new(shape, GEAR_THICKNES);

    Mesh::from(extrusion)
}
