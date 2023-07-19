pub mod camera;
pub mod object;
pub mod sphere;

use crate::{
    color::{self, Color},
    ray::{HitRecord, Ray},
};
use glam::Vec3;

use self::{camera::PerspectiveCamera, object::Shape, sphere::Sphere};

pub struct Scene {
    pub camera: PerspectiveCamera,
    spheres: Vec<Sphere>,
}

impl Scene {
    pub fn new(camera: PerspectiveCamera) -> Self {
        Self {
            camera,
            spheres: Vec::new(),
        }
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }

    pub fn render_pixel(&self, h: f32, v: f32) -> Color {
        let ray = self.camera.get_ray(h, v);
        self.trace_ray(&ray)
    }

    pub fn first_hit(&self, ray: &Ray) -> Option<HitRecord> {
        let mut t_min = f32::MAX;
        let mut result = None;

        for sphere in self.spheres.iter() {
            if let Some(record) = sphere.hit(ray, 0.0, t_min) {
                if record.t < t_min {
                    t_min = record.t;
                    result = Some(record);
                }
            }
        }

        result
    }

    pub fn trace_ray(&self, ray: &Ray) -> Color {
        let hit = self.first_hit(ray);
        if let Some(record) = hit {
            Color::from_normal(record.normal)
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

impl Default for Scene {
    fn default() -> Self {
        let mut scene = Self {
            camera: Default::default(),
            spheres: Default::default(),
        };

        scene.add_sphere(Sphere {
            center: Vec3::new(0.0, 0.0, 0.0),
            radius: 0.5,
        });

        scene.add_sphere(Sphere {
            center: Vec3::new(0.0, -100.5, 0.0),
            radius: 100.0,
        });

        scene
    }
}
