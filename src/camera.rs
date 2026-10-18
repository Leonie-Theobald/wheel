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
    pub(crate) fn init() -> Self {
        Self {
            yaw: 0.7,
            pitch: 0.45,
        }
    }

    pub(crate) fn get_position(&self) -> Vec3 {
        Vec3::new(
            CAMERA_DISTANCE * self.yaw.sin() * self.pitch.cos(),
            CAMERA_DISTANCE * self.pitch.sin(),
            CAMERA_DISTANCE * self.yaw.cos() * self.pitch.cos(),
        )
    }
}

fn rotate_camera(
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

    for (mut transform, mut orbit_camera) in &mut cameras {
        orbit_camera.yaw -= delta.x * SENSITIVITY;
        orbit_camera.pitch += delta.y * SENSITIVITY;

        // Prevent flipping over the poles
        orbit_camera.pitch = orbit_camera.pitch.clamp(-1.5, 1.5);

        transform.translation = orbit_camera.get_position();

        // Always look at the world origin
        transform.look_at(TARGET_CAMERA_VIEW, Vec3::Y);
    }
}
pub(crate) fn orbit_camera(
    mouse_motion: Res<AccumulatedMouseMotion>,
    buttons: Res<ButtonInput<MouseButton>>,
    cameras: Query<(&mut Transform, &mut OrbitCamera)>,
) {
    rotate_camera(mouse_motion, buttons, cameras);
}
