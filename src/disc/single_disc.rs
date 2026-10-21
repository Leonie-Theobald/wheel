use bevy::prelude::*;

#[derive(Clone, Component)]
pub(crate) struct SingleDisc {
    radius: f32,
    rotation: u8,
}

impl SingleDisc {
    pub(crate) fn init(radius: f32) -> Self {
        Self {
            radius,
            rotation: 0,
        }
    }

    pub(crate) fn get_radius(&self) -> f32 {
        self.radius
    }

    pub(crate) fn get_rotation_angle(&self) -> f32 {
        self.rotation as f32 * std::f32::consts::TAU / 16.0
    }

    pub(crate) fn step_once_clockwise(&mut self) {
        self.rotation = (self.rotation + 1) % 16;
    }

    pub(crate) fn step_once_counter_clockwise(&mut self) {
        // use +15 instead of -1 to avoid underflow
        self.rotation = (self.rotation + 15) % 16;
    }
}
