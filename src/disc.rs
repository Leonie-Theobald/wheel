use bevy::prelude::*;

const COLOR_SELECTED: Color = Color::srgb(0.95, 0.35, 0.1);
const COLOR_NOT_SELECTED: Color = Color::srgb(0.2, 0.6, 0.9);

pub(crate) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut discs: ResMut<Discs>,
) {
    let discs_definition = [
        Disc::init(5.),
        Disc::init(10.),
        Disc::init(15.),
        Disc::init(20.),
    ];

    // go through discs
    let mut height_offset = 0.0;
    for (index, disc) in discs_definition.iter().enumerate() {
        let color = if index == 0 {
            COLOR_SELECTED
        } else {
            COLOR_NOT_SELECTED
        };
        let entity = commands
            .spawn((
                disc.clone(),
                Mesh3d(meshes.add(Cylinder::new(disc.get_radius(), 1.))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: color,
                    ..default()
                })),
                Transform::from_xyz(0., height_offset, 0.)
                    // temporarily use a stretched circle to see rotations better
                    .with_scale(Vec3::new(1.0, 1.0, 0.3)),
            ))
            .id();

        discs.entities.push(entity);

        height_offset -= 1.1;
    }
}

#[derive(Clone, Component)]
pub(crate) struct Disc {
    radius: f32,
    rotation: u8,
}

impl Disc {
    pub(crate) fn init(radius: f32) -> Self {
        Self {
            radius,
            rotation: 0,
        }
    }

    pub(crate) fn get_radius(&self) -> f32 {
        self.radius
    }
}

#[derive(Default)]
pub(crate) enum SelectedDisc {
    #[default]
    Top,
    UpperMiddle,
    LowerMiddle,
    Bottom,
}

#[derive(Default, Resource)]
pub(crate) struct Discs {
    entities: Vec<Entity>,
    selected_disc: SelectedDisc,
}

impl Discs {
    // pub(crate) fn init(entities: [Entity; 4]) -> Self {
    //     Self {
    //         entities: Vec::from(entities),
    //         selected_disc: SelectedDisc::Top,
    //     }
    // }

    pub(crate) fn get_selected_disc(&self) -> &SelectedDisc {
        &self.selected_disc
    }

    pub(crate) fn set_selected_disc(&mut self, new_selected_disc: SelectedDisc) {
        self.selected_disc = new_selected_disc;
    }
}

pub(crate) fn toggle_active_states(
    keys: Res<ButtonInput<KeyCode>>,
    mut discs: ResMut<Discs>,
    mut disc_list: Query<(&mut Disc, &MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !keys.just_pressed(KeyCode::KeyC) {
        return;
    }

    // go through discs and reset optic for all discs
    for (_disc, material_handle) in &disc_list {
        if let Some(mut material) = materials.get_mut(&material_handle.0) {
            material.base_color = Color::srgb(0.2, 0.6, 0.9);
        }
    }

    // change what disc is active
    let new_selected_disc_entity = match discs.get_selected_disc() {
        SelectedDisc::Top => {
            discs.set_selected_disc(SelectedDisc::UpperMiddle);
            discs.entities[1]
        }
        SelectedDisc::UpperMiddle => {
            discs.set_selected_disc(SelectedDisc::LowerMiddle);
            discs.entities[2]
        }
        SelectedDisc::LowerMiddle => {
            discs.set_selected_disc(SelectedDisc::Bottom);
            discs.entities[3]
        }
        SelectedDisc::Bottom => {
            discs.set_selected_disc(SelectedDisc::Top);
            discs.entities[0]
        }
    };

    // change optic for selected disc
    let (_selected_disc, material_handle) = disc_list.get_mut(new_selected_disc_entity).unwrap();
    if let Some(mut material) = materials.get_mut(&material_handle.0) {
        material.base_color = COLOR_SELECTED;
    }
}

pub(crate) fn step_in_rotation(
    keys: Res<ButtonInput<KeyCode>>,
    discs: ResMut<Discs>,
    mut disc_list: Query<(&mut Disc, &mut Transform)>,
) {
    if !keys.just_pressed(KeyCode::KeyR) {
        return;
    }

    let selected_disc_entity = match discs.get_selected_disc() {
        SelectedDisc::Top => discs.entities[0],
        SelectedDisc::UpperMiddle => discs.entities[1],
        SelectedDisc::LowerMiddle => discs.entities[2],
        SelectedDisc::Bottom => discs.entities[3],
    };

    // change rotation for selected disc
    let (mut selected_disc, mut transform) = disc_list.get_mut(selected_disc_entity).unwrap();
    // Update the rotation position
    selected_disc.rotation = (selected_disc.rotation + 1) % 16;

    // 2 * PI / 16 = 22.5 => it takes 16 steps each 22.5 rad to make a full rotation
    // 2 * PI = TAU
    transform.rotate_y(std::f32::consts::TAU / 16.0);
}
