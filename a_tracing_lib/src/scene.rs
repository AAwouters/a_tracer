pub mod camera;
pub mod light;
pub mod object;
pub mod sphere;

use crate::{
    color::{self, Color},
    ray::{HitRecord, Ray},
};
use glam::Vec3;

use self::{
    camera::PerspectiveCamera,
    light::{ambient_light::AmbientLight, DirectionalLight, Light},
    object::Object,
    sphere::Sphere,
};

pub struct Scene {
    pub camera: PerspectiveCamera,
    objects: Vec<Object>,
    lights: Vec<Box<dyn Light>>,
}

impl Scene {
    pub fn new(camera: PerspectiveCamera) -> Self {
        Self {
            camera,
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn add_light<T: Light + 'static>(&mut self, light: T) {
        self.lights.push(Box::new(light));
    }

    pub fn render_pixel(&self, h: f32, v: f32) -> Color {
        let ray = self.camera.get_ray(h, v);
        self.trace_ray(&ray)
    }

    pub fn first_hit(&self, ray: &Ray) -> Option<(&Object, HitRecord)> {
        let mut t_min = f32::MAX;
        let mut result = None;

        for object in self.objects.iter() {
            if let Some(record) = object.shape.hit(ray, 0.0001, t_min) {
                if record.t < t_min {
                    t_min = record.t;
                    result = Some((object, record));
                }
            }
        }

        result
    }

    pub fn any_hit(&self, ray: &Ray) -> Option<HitRecord> {
        let t_min = f32::MAX;

        for object in self.objects.iter() {
            if let Some(record) = object.shape.hit(ray, 0.0001, t_min) {
                if record.t < t_min {
                    return Some(record);
                }
            }
        }

        None
    }

    pub fn trace_ray(&self, ray: &Ray) -> Color {
        let hit = self.first_hit(ray);
        if let Some((object, record)) = hit {
            let mut color_sum = color::BLACK;

            for light in self.lights.iter() {
                if let Some(light_ray) = light.light_at(self, record.point) {
                    let camera_dir = (self.camera.origin - record.point).normalize();
                    color_sum += object.shade(
                        record.normal,
                        light_ray.color,
                        light_ray.direction,
                        camera_dir,
                    );
                }
            }

            color_sum
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
        let mut scene = Self::new(PerspectiveCamera::default());

        scene.add_object(Object::new(
            Sphere {
                center: Vec3::new(0.0, 0.0, 0.0),
                radius: 0.5,
            },
            color::RED,
        ));

        scene.add_object(Object::new(
            Sphere {
                center: Vec3::new(0.0, -100.5, 0.0),
                radius: 100.0,
            },
            color::BLUE,
        ));

        scene.add_light(DirectionalLight::new(
            color::WHITE,
            Vec3::new(0.1, -0.7, -0.2),
        ));

        scene.add_light(AmbientLight::new(Color::new(0.2, 0.2, 0.2)));

        scene
    }
}
