use bevy::prelude::*;

pub(crate) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let discs = [
        Disc::init(5., true),
        Disc::init(10., false),
        Disc::init(15., false),
    ];
    let mut height_offset = 0.0;

    // separately spawn first disc to make it active disc afterwards
    let first_disc = discs.first().expect("Just defined a non-empty disc array");
    let active_disc_entity = commands
        .spawn((
            first_disc.clone(),
            Mesh3d(meshes.add(Cylinder::new(first_disc.get_radius(), 1.))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: first_disc.get_color(),
                ..default()
            })),
            Transform::from_xyz(0., height_offset, 0.)
                // temporarily use a stretched circle to see rotations better
                .with_scale(Vec3::new(1.0, 1.0, 0.3)),
        ))
        .id();
    height_offset -= 1.1;
    commands.insert_resource(ActiveDisc::init(active_disc_entity));

    // go through other discs
    for disc in discs.iter().skip(1) {
        commands.spawn((
            disc.clone(),
            Mesh3d(meshes.add(Cylinder::new(disc.get_radius(), 1.))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: disc.get_color(),
                ..default()
            })),
            Transform::from_xyz(0., height_offset, 0.)
                // temporarily use a stretched circle to see rotations better
                .with_scale(Vec3::new(1.0, 1.0, 0.3)),
        ));

        height_offset -= 1.1;
    }
}

#[derive(Clone, Component)]
pub(crate) struct Disc {
    radius: f32,
    active: bool,
    rotation: u8,
}

impl Disc {
    pub(crate) fn init(radius: f32, active: bool) -> Self {
        Self {
            radius,
            active,
            rotation: 0,
        }
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

#[derive(Default, Resource)]
pub(crate) struct ActiveDisc {
    entity: Option<Entity>,
}

impl ActiveDisc {
    pub(crate) fn init(entity: Entity) -> Self {
        Self {
            entity: Some(entity),
        }
    }
}
pub(crate) fn toggle_active_states(
    keys: Res<ButtonInput<KeyCode>>,
    mut discs: Query<(&mut Disc, &MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !keys.just_pressed(KeyCode::KeyC) {
        return;
    }

    for (mut disc, material_handle) in &mut discs {
        // Switch state
        disc.active = !disc.active;

        // Update the rendered material
        if let Some(mut material) = materials.get_mut(&material_handle.0) {
            material.base_color = disc.get_color();
        }
    }
}

pub(crate) fn step_in_rotation(
    keys: Res<ButtonInput<KeyCode>>,
    mut discs: Query<(&mut Disc, &mut Transform)>,
) {
    if !keys.just_pressed(KeyCode::KeyR) {
        return;
    }

    for (mut disc, mut transform) in &mut discs {
        if !disc.active {
            continue;
        }

        // Update the rotation position
        disc.rotation = (disc.rotation + 1) % 16;

        // 2 * PI / 16 = 22.5 => it takes 16 steps to make a full rotation
        // 2 * PI = TAU
        transform.rotate_y(std::f32::consts::TAU / 16.0);
    }
}
