use std::f32::consts::FRAC_PI_3;

use glam::Vec3;

use crate::ray::Ray;

pub struct PerspectiveCamera {
    pub origin: Vec3,
    direction: Vec3,
    lower_left_corner: Vec3,
    up: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    vertical_fov: f32,
    aspect_ratio: f32,
}

impl PerspectiveCamera {
    /// Create a new perspective camera from the given parameters
    /// fov is in radians
    /// aspect ratio is width / height
    pub fn new(
        origin: Vec3,
        direction: Vec3,
        up: Vec3,
        vertical_fov: f32,
        aspect_ratio: f32,
    ) -> Self {
        let h = (vertical_fov * 0.5).tan();
        let vp_height = 2.0 * h;
        let vp_width = aspect_ratio * vp_height;

        // We have to invert the direction because thats simply how the math works
        let w = -direction.normalize();
        let u = up.cross(w).normalize();
        let v = w.cross(u);

        let horizontal = vp_width * u;
        let vertical = vp_height * v;
        let lower_left_corner = origin - 0.5 * horizontal - 0.5 * vertical - w;

        Self {
            origin,
            direction: direction,
            lower_left_corner,
            up,
            horizontal,
            vertical,
            vertical_fov,
            aspect_ratio,
        }
    }

    /// Get a ray from the given horizontal and vertical values
    /// h and v are expected to be in the range 0.0 to 1.0 and represent the relative distance from the bottom left corner of the viewport
    pub fn get_ray(&self, h: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + h * self.horizontal + v * self.vertical
                - self.origin,
        }
    }

    fn recalculate_parameters(&mut self) {
        let h = (self.vertical_fov * 0.5).tan();
        let vp_height = 2.0 * h;
        let vp_width = self.aspect_ratio * vp_height;

        let w = -self.direction;
        let u = self.up.cross(w).normalize();
        let v = w.cross(u);

        self.horizontal = vp_width * u;
        self.vertical = vp_height * v;
        self.lower_left_corner = self.origin - 0.5 * self.horizontal - 0.5 * self.vertical - w;
    }

    /// Set the aspect ratio of the camera to the given value
    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
        self.recalculate_parameters();
    }

    /// Set the vertical field of view of the camera to the given value
    pub fn set_vertical_fov(&mut self, vertical_fov: f32) {
        self.vertical_fov = vertical_fov;
        self.recalculate_parameters();
    }

    /// Set the vertical field of view of the camera depending on the given horizontal field of view and the stored aspect ratio
    pub fn set_horizontal_fov(&mut self, horizontal_fov: f32) {
        self.vertical_fov = 1.0 / self.aspect_ratio * horizontal_fov;
    }

    /// Set the position of the camera to the given value
    pub fn set_origin(&mut self, origin: Vec3) {
        self.origin = origin;
        self.recalculate_parameters();
    }

    /// Set the viewing direction of the camera to the given value
    pub fn set_direction(&mut self, direction: Vec3) {
        self.direction = direction.normalize();
        self.recalculate_parameters();
    }

    /// Set both the position and the viewing direction of the camera
    pub fn set_origin_and_direction(&mut self, origin: Vec3, direction: Vec3) {
        self.origin = origin;
        self.direction = direction.normalize();
        self.recalculate_parameters();
    }

    pub fn apply_movement(
        &mut self,
        forward: f32,
        sideways: f32,
        vertical: f32,
        pitch: f32,
        yaw: f32,
    ) {
        let forward_vector = -Vec3::new(self.direction.x, 0.0, self.direction.z).normalize();
        let sideways_vector = forward_vector.cross(self.up);
        let camera_movement =
            -forward * forward_vector + sideways * sideways_vector + vertical * self.up;

        let mut target_direction = self.direction;
        if yaw.abs() > 0.01 {
            target_direction = rotate_vector(target_direction, self.up, yaw);
        }
        if pitch.abs() > 0.01 {
            target_direction = rotate_vector(target_direction, sideways_vector, pitch);
        }

        // self.set_origin(self.origin + camera_movement);
        self.set_origin_and_direction(self.origin + camera_movement, target_direction);
    }
}

impl Default for PerspectiveCamera {
    fn default() -> Self {
        Self::new(
            Vec3::new(0.0, 0.0, -5.0),
            Vec3::Z,
            Vec3::Y,
            FRAC_PI_3,
            16.0 / 9.0,
        )
    }
}

fn rotate_vector(vector: Vec3, axis: Vec3, angle: f32) -> Vec3 {
    let cos_factor = angle.cos();
    let cos_vector = vector * cos_factor;

    let sin_factor = angle.sin();
    let sin_vector = axis.cross(vector) * sin_factor;

    let k_vector = axis.dot(vector) * (1.0 - cos_factor) * vector;

    cos_vector + sin_vector + k_vector
}
