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
    pub(crate) fn init_gear(
        radius: f32,
        outer_numbers: [u8; 8],
        inner_numbers: Option<[u8; 16]>,
    ) -> Self {
        let gear = Gear::init(radius, outer_numbers, inner_numbers);

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
        // spawn disc (full disc or gear)
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

        // Get numbers and their positions
        let mut numbers_and_positions = Vec::new();
        let mut rotational_offset = ROTATION_STEP / 2.0;
        let outer_radius = self.get_radius() - 0.5 * RING_HEIGHT;
        let inner_radius = self.get_radius() - 1.5 * RING_HEIGHT;

        for (outer_number, inner_number) in self
            .get_outer_numbers()
            .into_iter()
            .zip(self.get_inner_numbers())
        {
            // adjust orientation of number to face disc center
            let rotation = Quat::from_rotation_y(rotational_offset);

            for (number, radius) in [(outer_number, outer_radius), (inner_number, inner_radius)] {
                if let Some(number) = number {
                    let translation = Vec3::new(
                        radius * rotational_offset.sin(),
                        DISC_THICKNESS / 2.0 + 0.04,
                        radius * rotational_offset.cos(),
                    );

                    numbers_and_positions.push((number, translation, rotation));
                }
            }

            rotational_offset += ROTATION_STEP;
        }

        // spawn the numbers
        for (number, translation, rotation) in numbers_and_positions {
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
                    base_color: Color::srgb(0.01, 0.01, 0.01),
                    unlit: true,
                    ..default()
                })),
                Transform {
                    translation,
                    rotation: rotation * Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
                    scale: Vec3::splat(0.8),
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
        .rotated_by(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2))
    }

    pub(crate) fn get_radius(&self) -> f32 {
        match &self.disc {
            Disc::Gear(gear) => gear.get_radius(),
            Disc::FullDisc(full_disc) => full_disc.get_radius(),
        }
    }

    fn get_outer_numbers(&self) -> [Option<u8>; 16] {
        match self.disc {
            Disc::Gear(ref gear) => gear.get_outer_numbers(),
            Disc::FullDisc(ref full_disc) => full_disc.get_outer_numbers(),
        }
    }

    fn get_inner_numbers(&self) -> [Option<u8>; 16] {
        match self.disc {
            Disc::Gear(ref gear) => gear.get_inner_numbers(),
            Disc::FullDisc(ref full_disc) => full_disc.get_inner_numbers(),
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
