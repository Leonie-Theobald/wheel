use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;

pub(crate) const TARGET_CAMERA_VIEW: Vec3 = Vec3::ZERO;
const SENSITIVITY: f32 = 0.005;
const CAMERA_DISTANCE: f32 = 20.;

#[derive(Component)]
pub(crate) struct OrbitCamera {
    yaw: f32,
    pitch: f32,
}

impl OrbitCamera {
    pub(crate) fn init(yaw: f32, pitch: f32) -> Self {
        Self { yaw, pitch }
    }
}

pub(crate) fn orbit_position(yaw: f32, pitch: f32) -> Vec3 {
    Vec3::new(
        CAMERA_DISTANCE * yaw.sin() * pitch.cos(),
        CAMERA_DISTANCE * pitch.sin(),
        CAMERA_DISTANCE * yaw.cos() * pitch.cos(),
    )
}

pub(crate) fn orbit_camera(
    mouse_motion: Res<AccumulatedMouseMotion>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut cameras: Query<(&mut Transform, &mut OrbitCamera)>,
) {
    if !buttons.pressed(MouseButton::Left) {
        return;
    }

    let delta = mouse_motion.delta;

    if delta == Vec2::ZERO {
        return;
    }

    for (mut transform, mut orbit) in &mut cameras {
        orbit.yaw -= delta.x * SENSITIVITY;
        orbit.pitch += delta.y * SENSITIVITY;

        // Prevent flipping over the poles
        orbit.pitch = orbit.pitch.clamp(-1.5, 1.5);

        transform.translation = orbit_position(orbit.yaw, orbit.pitch);

        // Always look at the world origin
        transform.look_at(TARGET_CAMERA_VIEW, Vec3::Y);
    }
}
