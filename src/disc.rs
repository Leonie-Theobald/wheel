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

    discs.selected_disc = SelectedDisc::Top;
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
    pub(crate) fn get_selected_disc_entity(&self) -> Entity {
        match &self.selected_disc {
            SelectedDisc::Top => self.entities[0],
            SelectedDisc::UpperMiddle => self.entities[1],
            SelectedDisc::LowerMiddle => self.entities[2],
            SelectedDisc::Bottom => self.entities[3],
        }
    }

    /// The discs are vertically stacked \
    /// This function makes the disc above the currently selected disc the newly selected disc \
    /// If the currently selected disc is the top one, it wraps around to the bottom disc
    pub(crate) fn select_next_upper_disc(&mut self) {
        match self.selected_disc {
            SelectedDisc::Top => {
                self.selected_disc = SelectedDisc::Bottom;
            }
            SelectedDisc::UpperMiddle => {
                self.selected_disc = SelectedDisc::Top;
            }
            SelectedDisc::LowerMiddle => {
                self.selected_disc = SelectedDisc::UpperMiddle;
            }
            SelectedDisc::Bottom => {
                self.selected_disc = SelectedDisc::LowerMiddle;
            }
        };
    }

    /// The discs are vertically stacked \
    /// This function makes the disc below the currently selected disc the newly selected disc \
    /// If the currently selected disc is the bottom one, it wraps around to the top disc
    pub(crate) fn select_next_lower_disc(&mut self) {
        match self.selected_disc {
            SelectedDisc::Top => {
                self.selected_disc = SelectedDisc::UpperMiddle;
            }
            SelectedDisc::UpperMiddle => {
                self.selected_disc = SelectedDisc::LowerMiddle;
            }
            SelectedDisc::LowerMiddle => {
                self.selected_disc = SelectedDisc::Bottom;
            }
            SelectedDisc::Bottom => {
                self.selected_disc = SelectedDisc::Top;
            }
        };
    }
}

pub(crate) fn toggle_active_states(keys: Res<ButtonInput<KeyCode>>, mut discs: ResMut<Discs>) {
    if keys.just_pressed(KeyCode::ArrowUp) {
        discs.select_next_upper_disc();
    }
    if keys.just_pressed(KeyCode::ArrowDown) {
        discs.select_next_lower_disc();
    }
}

pub(crate) fn step_in_rotation(
    keys: Res<ButtonInput<KeyCode>>,
    discs: Res<Discs>,
    mut disc_list: Query<&mut Disc>,
) {
    let selected_disc_entity = discs.get_selected_disc_entity();
    let mut selected_disc = disc_list.get_mut(selected_disc_entity).unwrap();

    if keys.just_pressed(KeyCode::ArrowLeft) {
        selected_disc.step_once_counter_clockwise();
    }
    if keys.just_pressed(KeyCode::ArrowRight) {
        selected_disc.step_once_clockwise();
    }
}

pub(crate) fn color_discs(
    discs: ResMut<Discs>,
    mut disc_list: Query<&MeshMaterial3d<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // only necessary when the selected disc has changed
    if !discs.is_changed() {
        return;
    }

    // reset color optic and show correct rotation of all discs
    for material_handle in &mut disc_list {
        materials.get_mut(&material_handle.0).unwrap().base_color = COLOR_NOT_SELECTED;
    }

    // change color for selected disc
    let selected_disc_entity = discs.get_selected_disc_entity();
    let material_handle = disc_list.get(selected_disc_entity).unwrap();
    materials.get_mut(&material_handle.0).unwrap().base_color = COLOR_SELECTED;
}

pub(crate) fn rotate_discs(mut disc_list: Query<(&Disc, &mut Transform), Changed<Disc>>) {
    for (disc, mut transform) in &mut disc_list {
        transform.rotation = Quat::from_rotation_y(disc.get_rotation_angle());
    }
}
