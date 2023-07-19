pub mod camera;

use crate::{
    color::{self, Color},
    ray::Ray,
};
use glam::Vec3;

use self::camera::PerspectiveCamera;

#[derive(Default)]
pub struct Scene {
    camera: PerspectiveCamera,
}

impl Scene {
    pub fn new(camera: PerspectiveCamera) -> Self {
        Self { camera }
    }

    pub fn render_pixel(&self, h: f32, v: f32) -> Color {
        let ray = self.camera.get_ray(h, v);
        self.trace_ray(&ray)
    }

    pub fn trace_ray(&self, ray: &Ray) -> Color {
        let hit = false;
        if hit {
            color::RED
        } else {
            self.background_color(&ray.direction)
        }
    }

    pub fn background_color(&self, direction: &Vec3) -> Color {
        let normalized = direction.normalize();
        let t = 0.5 * (normalized.y + 1.0);
        color::SKYBLUE.lerp(color::WHITE, t)
    }
}
