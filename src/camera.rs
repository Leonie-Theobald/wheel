use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
use bevy::prelude::*;

pub(crate) const TARGET_CAMERA_VIEW: Vec3 = Vec3::ZERO;
const SENSITIVITY_ROTATION: f32 = 0.005;
const SENSITIVITY_ZOOM: f32 = 0.02;
const CAMERA_DISTANCE_MIN: f32 = 10.;
const CAMERA_DISTANCE_MAX: f32 = 50.;

#[derive(Component)]
pub(crate) struct OrbitCamera {
    yaw: f32,
    pitch: f32,
    camera_distance: f32,
}

impl OrbitCamera {
    pub(crate) fn init() -> Self {
        Self {
            yaw: 0.7,
            pitch: 0.45,
            camera_distance: CAMERA_DISTANCE_MAX,
        }
    }

    pub(crate) fn get_position(&self) -> Vec3 {
        Vec3::new(
            self.camera_distance * self.yaw.sin() * self.pitch.cos(),
            self.camera_distance * self.pitch.sin(),
            self.camera_distance * self.yaw.cos() * self.pitch.cos(),
        )
    }
}

fn rotate_camera(
    mouse_motion: &AccumulatedMouseMotion,
    buttons: &ButtonInput<MouseButton>,
    cameras: &mut Query<(&mut Transform, &mut OrbitCamera)>,
) {
    if !buttons.pressed(MouseButton::Left) {
        return;
    }

    let delta = mouse_motion.delta;

    if delta == Vec2::ZERO {
        return;
    }

    for (mut transform, mut orbit_camera) in cameras {
        orbit_camera.yaw -= delta.x * SENSITIVITY_ROTATION;
        orbit_camera.pitch += delta.y * SENSITIVITY_ROTATION;

        // Prevent flipping over the poles
        orbit_camera.pitch = orbit_camera.pitch.clamp(-1.5, 1.5);

        transform.translation = orbit_camera.get_position();

        // Always look at the world origin
        transform.look_at(TARGET_CAMERA_VIEW, Vec3::Y);
    }
}

fn zoom_camera(
    mouse_scroll: &AccumulatedMouseScroll,
    cameras: &mut Query<(&mut Transform, &mut OrbitCamera)>,
) {
    let scroll = mouse_scroll.delta.y;

    for (mut transform, mut orbit_camera) in cameras {
        if scroll != 0.0 {
            orbit_camera.camera_distance *= (-scroll * SENSITIVITY_ZOOM).exp();
            orbit_camera.camera_distance = orbit_camera
                .camera_distance
                .clamp(CAMERA_DISTANCE_MIN, CAMERA_DISTANCE_MAX);
        }

        // Recalculate the camera position
        transform.translation = orbit_camera.get_position();

        // Always look at the world origin
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}

pub(crate) fn orbit_camera(
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut cameras: Query<(&mut Transform, &mut OrbitCamera)>,
) {
    zoom_camera(&mouse_scroll, &mut cameras);
    rotate_camera(&mouse_motion, &buttons, &mut cameras);
}
