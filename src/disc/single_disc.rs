use super::full_disc::FullDisc;
use super::gear::Gear;
use bevy::prelude::*;

pub(crate) const DISC_THICKNESS: f32 = 0.5;
pub(crate) const RING_HEIGHT: f32 = 1.;

#[derive(Clone)]
enum Disc {
    Gear(Gear),
    FullDisc(FullDisc),
}

#[derive(Clone, Component)]
pub(crate) struct SingleDisc {
    disc: Disc,
    rotation: u8,
}

impl SingleDisc {
    pub(crate) fn init_gear(radius: f32) -> Self {
        let gear = Gear::init(radius);

        Self {
            disc: Disc::Gear(gear),
            rotation: 0,
        }
    }

    pub(crate) fn init_full_disc(radius: f32) -> Self {
        let full_disc = FullDisc::init(radius);

        Self {
            disc: Disc::FullDisc(full_disc),
            rotation: 0,
        }
    }

    pub(crate) fn generate_mesh(&self) -> Mesh {
        match &self.disc {
            Disc::Gear(gear) => gear.generate_mesh(),
            Disc::FullDisc(full_disc) => full_disc.generate_mesh(),
        }
        .rotated_by(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
    }

    pub(crate) fn get_radius(&self) -> f32 {
        match &self.disc {
            Disc::Gear(gear) => gear.get_radius_inner(),
            Disc::FullDisc(full_disc) => full_disc.get_radius(),
        }
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
