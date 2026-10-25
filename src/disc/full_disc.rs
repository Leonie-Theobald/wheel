use crate::disc::single_disc::DISC_THICKNESS;
use bevy::prelude::*;

#[derive(Clone)]
pub(crate) struct FullDisc {
    radius: f32,
}

impl FullDisc {
    pub(crate) fn init(radius: f32) -> Self {
        Self { radius }
    }

    pub(crate) fn get_radius(&self) -> f32 {
        self.radius
    }

    pub(crate) fn generate_mesh(&self) -> Mesh {
        Mesh::from(Extrusion::new(Circle::new(self.radius), DISC_THICKNESS))
    }
}
