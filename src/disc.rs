use bevy::prelude::*;

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
