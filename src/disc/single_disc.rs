use super::full_disc::FullDisc;
use super::gear::Gear;
use bevy::prelude::*;
use bevy_fontmesh::{JustifyText, TextAnchor, TextMesh, TextMeshStyle};

pub(crate) const COUNT_NUMBERS: usize = 16;
pub(crate) const ROTATION_STEP: f32 = std::f32::consts::TAU / COUNT_NUMBERS as f32;
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

    pub(crate) fn init_full_disc(
        radius: f32,
        outer_numbers: [u8; 16],
        inner_numbers: [u8; 16],
    ) -> Self {
        let full_disc = FullDisc::init(radius, outer_numbers, inner_numbers);

        Self {
            disc: Disc::FullDisc(full_disc),
            rotation: 0,
        }
    }

    pub(crate) fn spawn(
        &self,
        color: Color,
        height_offset: f32,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Entity {
        let entity = commands
            .spawn((
                self.clone(),
                Mesh3d(meshes.add(self.generate_disc_mesh())),
                MeshMaterial3d(materials.add(StandardMaterial {
                    cull_mode: None,
                    base_color: color,
                    ..default()
                })),
                Transform::from_xyz(0., height_offset, 0.0),
            ))
            .id();

        for (number, translation, rotation) in self.get_numbers_and_position() {
            // Spawn writing on disc
            commands.spawn((
                TextMesh {
                    text: number.to_string(),
                    style: TextMeshStyle {
                        depth: 0.08,
                        anchor: TextAnchor::Center,
                        justify: JustifyText::Center,
                        ..default()
                    },
                    ..default()
                },
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::BLACK,
                    ..default()
                })),
                Transform {
                    // translation: Vec3::new(0.0, DISC_THICKNESS / 2. + 0.08, 0.0),
                    translation,
                    // rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
                    rotation: rotation * Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
                    scale: Vec3::splat(1.),
                },
                ChildOf(entity),
            ));
        }

        entity
    }

    fn generate_disc_mesh(&self) -> Mesh {
        match &self.disc {
            Disc::Gear(gear) => gear.generate_gear_mesh(),
            Disc::FullDisc(full_disc) => full_disc.generate_full_disc_mesh(),
        }
        .rotated_by(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
    }

    // fn generate_numbers_meshes(&self) -> Vec<TextMesh> {
    //     match &self.disc {
    //         Disc::Gear(gear) => gear.generate_numbers_meshes(),
    //         Disc::FullDisc(full_disc) => full_disc.generate_numbers_meshes(),
    //     }
    // }
    fn get_numbers_and_position(&self) -> Vec<(u8, Vec3, Quat)> {
        match &self.disc {
            Disc::Gear(gear) => gear.get_numbers_and_positions(),
            Disc::FullDisc(full_disc) => full_disc.get_numbers_and_positions(),
        }
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
