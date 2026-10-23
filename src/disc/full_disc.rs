#[derive(Clone)]
pub(crate) struct FullDisc {
    radius: f32,
}

impl FullDisc {
    pub(crate) fn init(radius: f32) -> Self {
        Self { radius }
    }

    pub(crate) fn get_radius(&self) -> f32 {
        self.radius
    }
}
