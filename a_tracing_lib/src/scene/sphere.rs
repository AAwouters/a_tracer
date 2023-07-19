use glam::Vec3;

use crate::ray::{HitRecord, Ray};

use super::object::Shape;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Shape for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<crate::ray::HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let hb = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let disc = hb * hb - a * c;

        if disc < 0.0 {
            return None;
        }

        let sqrt_disc = disc.sqrt();
        let mut root = (-hb - sqrt_disc) / a;

        if root < t_min || t_max < root {
            root = (-hb + sqrt_disc) / a;

            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let point = ray.at(t);
        let normal = (point - self.center) / self.radius;

        Some(HitRecord { point, normal, t })
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec3;

    use crate::{ray::Ray, scene::object::Shape};

    use super::Sphere;

    #[test]
    fn hit_test() {
        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, 2.0),
            direction: Vec3::NEG_Z,
        };

        let sphere = Sphere {
            center: Vec3::ZERO,
            radius: 1.0,
        };

        let hit_record = sphere.hit(&ray, 0.0, f32::MAX);
        assert!(hit_record.is_some());
    }
}
