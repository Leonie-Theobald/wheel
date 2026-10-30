use crate::AllDiscs;
use crate::disc::single_disc::SingleDisc;
use crate::disc::single_disc::{COUNT_NUMBERS, DISC_THICKNESS, RING_HEIGHT, ROTATION_STEP};
use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use bevy_fontmesh::{JustifyText, TextAnchor, TextMesh, TextMeshStyle};

#[derive(Resource)]
pub(crate) struct SumMaterial(pub Handle<StandardMaterial>);

#[derive(Default, Resource)]
pub(crate) struct AllSums {
    sums: [usize; COUNT_NUMBERS],
}

impl AllSums {
    fn reset_sums(&mut self) {
        self.sums = [0; COUNT_NUMBERS];
    }

    pub(crate) fn calculate_sums(
        &mut self,
        discs: &Res<AllDiscs>,
        disc_list: &Query<&mut SingleDisc>,
    ) {
        self.reset_sums();

        for ring in discs.get_visible_numbers(disc_list) {
            for (index, number) in ring.iter().enumerate() {
                self.sums[index] += number;
            }
        }
    }

    pub(crate) fn get_sums(&self) -> [usize; COUNT_NUMBERS] {
        self.sums
    }

    pub(crate) fn sums_equal(&self) -> bool {
        self.sums.iter().all(|sum| sum == &self.sums[0])
    }
}

#[derive(Default, Component, Debug)]
pub(crate) struct SumNumber {
    index: usize,
}

impl SumNumber {
    pub(crate) fn get_index(&self) -> usize {
        self.index
    }
}

pub(crate) fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    discs: Res<AllDiscs>,
    disc_list: Query<&mut SingleDisc>,
    mut all_sums: ResMut<AllSums>,
) {
    // Get sum numbers and their positions
    let mut numbers_and_positions = Vec::new();
    let mut rotational_offset = ROTATION_STEP / 2.0;

    let bottom_disc_entity = discs.get_bottom_disc_entity();
    let radius = disc_list.get(bottom_disc_entity).unwrap().get_radius() + RING_HEIGHT;

    all_sums.calculate_sums(&discs, &disc_list);
    for number in all_sums.sums {
        // adjust orientation of number to face disc center
        let rotation = Quat::from_rotation_y(rotational_offset);

        let translation = Vec3::new(
            radius * rotational_offset.sin(),
            DISC_THICKNESS / 2.0 + 0.04 - 4. * (DISC_THICKNESS + 0.1),
            radius * rotational_offset.cos(),
        );

        numbers_and_positions.push((number, translation, rotation));

        rotational_offset += ROTATION_STEP;
    }

    // spawn the numbers
    let sum_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1., 0., 0.),
        unlit: true,
        ..default()
    });
    commands.insert_resource(SumMaterial(sum_material.clone()));

    for (index, (number, translation, rotation)) in numbers_and_positions.into_iter().enumerate() {
        commands.spawn((
            SumNumber { index },
            Visibility::Visible,
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
            MeshMaterial3d(sum_material.clone()),
            Transform {
                translation,
                rotation: rotation * Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
                scale: Vec3::splat(0.8),
            },
            NotShadowCaster,
        ));
    }
}

pub(crate) fn toggle_visibility(
    keys: Res<ButtonInput<KeyCode>>,
    sums: Query<&mut Visibility, With<SumNumber>>,
) {
    if !keys.just_pressed(KeyCode::KeyS) {
        return;
    }

    for mut visibility in sums {
        *visibility = match *visibility {
            Visibility::Visible => Visibility::Hidden,
            Visibility::Hidden => Visibility::Visible,
            Visibility::Inherited => Visibility::Hidden,
        }
    }
}
