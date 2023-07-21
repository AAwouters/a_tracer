pub mod ambient_light;
pub mod directional_light;

use super::Scene;
use crate::color::Color;
pub use directional_light::DirectionalLight;
use glam::Vec3;

pub struct LightRay {
    pub direction: Option<Vec3>,
    pub color: Color,
}

pub trait Light {
    fn light_at(&self, scene: &Scene, location: Vec3) -> Option<LightRay>;
}
