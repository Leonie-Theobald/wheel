use bevy::prelude::*;

#[derive(Clone, Component)]
pub(crate) struct Disc {
    radius: f32,
    active: bool,
}

impl Disc {
    pub(crate) fn init(radius: f32, active: bool) -> Self {
        Self { radius, active }
    }

    pub(crate) fn get_radius(&self) -> f32 {
        self.radius
    }

    pub(crate) fn get_color(&self) -> Color {
        if self.active {
            Color::srgb(0.95, 0.35, 0.1)
        } else {
            Color::srgb(0.2, 0.6, 0.9)
        }
    }
}
