use glam::Vec3;

use crate::{
    color::Color,
    ray::{HitRecord, Ray},
};

pub trait Shape {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Object {
    pub shape: Box<dyn Shape>,
    pub color: Color,
}

impl Object {
    pub fn new<S: Shape + 'static>(shape: S, color: Color) -> Self {
        Self {
            shape: Box::new(shape),
            color,
        }
    }

    pub fn shade(
        &self,
        normal: Vec3,
        light_in: Color,
        light_direction: Option<Vec3>,
        out_direction: Vec3,
    ) -> Color {
        if let Some(light_direction) = light_direction {
            let intensity = normal.dot(light_direction).clamp(0.0, 1.0);
            let diffuse = intensity * light_in * self.color;

            let half_vector = (light_direction + out_direction).normalize();
            let n_dot_h = normal.dot(half_vector).clamp(0.0, 1.0);
            let specular_hardness = 2.0;
            let intensity = n_dot_h.powf(specular_hardness);
            let specular = intensity * light_in * self.color;

            diffuse + specular
        } else {
            light_in * self.color
        }
    }
}
