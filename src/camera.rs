use crate::lighting::Lighting;
use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
use bevy::prelude::*;

pub const TARGET_CAMERA_VIEW: Vec3 = Vec3::ZERO;
const SENSITIVITY_ROTATION: f32 = 0.005;
const SENSITIVITY_ZOOM: f32 = 0.02;
const CAMERA_DISTANCE_MIN: f32 = 10.;
const CAMERA_DISTANCE_MAX: f32 = 35.;

pub fn setup(mut commands: Commands) {
    let orbit_camera = OrbitCamera::init();
    let position = orbit_camera.get_position();

    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(position).looking_at(TARGET_CAMERA_VIEW, Vec3::Y),
        orbit_camera,
    ));
}

#[derive(Component)]
pub struct OrbitCamera {
    yaw: f32,
    pitch: f32,
    camera_distance: f32,
}

impl OrbitCamera {
    pub fn init() -> Self {
        Self {
            yaw: 0.7,
            pitch: 0.85,
            camera_distance: 0.75 * CAMERA_DISTANCE_MAX,
        }
    }

    pub fn get_position(&self) -> Vec3 {
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
    camera: &mut Single<(&mut Transform, &mut OrbitCamera), Without<Lighting>>,
    lightings: &mut Query<(&mut Transform, &Lighting), Without<OrbitCamera>>,
) {
    if !buttons.pressed(MouseButton::Left) {
        return;
    }

    let delta = mouse_motion.delta;

    if delta == Vec2::ZERO {
        return;
    }

    let (camera_transform, orbit_camera) = &mut **camera;
    orbit_camera.yaw -= delta.x * SENSITIVITY_ROTATION;
    orbit_camera.pitch += delta.y * SENSITIVITY_ROTATION;

    // Prevent flipping over the poles
    orbit_camera.pitch = orbit_camera.pitch.clamp(
        -std::f32::consts::FRAC_PI_2 + 0.01,
        std::f32::consts::FRAC_PI_2 - 0.01,
    );

    let camera_position = orbit_camera.get_position();
    camera_transform.translation = camera_position;

    // Always look at the world origin
    camera_transform.look_at(TARGET_CAMERA_VIEW, Vec3::Y);

    // Put lighting at the same position as camera
    for (mut lighting_transform, _lighting) in lightings {
        lighting_transform.translation = camera_position;
        lighting_transform.look_at(TARGET_CAMERA_VIEW, Vec3::Y);
    }
}

fn zoom_camera(
    mouse_scroll: &AccumulatedMouseScroll,
    camera: &mut Single<(&mut Transform, &mut OrbitCamera), Without<Lighting>>,
) {
    let scroll = mouse_scroll.delta.y;

    let (transform, orbit_camera) = &mut **camera;
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

pub fn orbit_camera(
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut camera: Single<(&mut Transform, &mut OrbitCamera), Without<Lighting>>,
    mut lightings: Query<(&mut Transform, &Lighting), Without<OrbitCamera>>,
) {
    zoom_camera(&mouse_scroll, &mut camera);
    rotate_camera(&mouse_motion, &buttons, &mut camera, &mut lightings);
}
