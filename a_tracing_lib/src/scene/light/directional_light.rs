use glam::Vec3;

use crate::{color::Color, ray::Ray};

use super::{Light, LightRay};

#[derive(Debug, Clone, Copy)]
pub struct DirectionalLight {
    color: Color,
    direction: Vec3,
}

impl DirectionalLight {
    pub fn new(color: Color, direction: Vec3) -> Self {
        Self {
            color,
            direction: direction.normalize(),
        }
    }
}

impl Light for DirectionalLight {
    fn light_at(&self, scene: &crate::scene::Scene, location: Vec3) -> Option<super::LightRay> {
        match scene.any_hit(&Ray {
            origin: location,
            direction: -self.direction,
        }) {
            Some(_) => None,
            None => Some(LightRay {
                direction: Some(-self.direction),
                color: self.color,
            }),
        }
    }
}
