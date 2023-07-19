use crate::ray::{HitRecord, Ray};

pub trait Shape {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
