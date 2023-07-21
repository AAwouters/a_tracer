use crate::color::Color;

use super::Light;

pub struct AmbientLight {
    color: Color,
}

impl AmbientLight {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Light for AmbientLight {
    fn light_at(
        &self,
        _scene: &crate::scene::Scene,
        _location: glam::Vec3,
    ) -> Option<super::LightRay> {
        Some(super::LightRay {
            direction: None,
            color: self.color,
        })
    }
}
