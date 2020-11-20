use super::vec3::{Point3, Ray, Vec3};
use std::f64::consts::PI;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

pub struct Degrees(pub f64);

impl Degrees {
    pub fn to_radians(&self) -> f64 {
        &self.0 * PI / 180.0
    }
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        vfov: Degrees,
        aspect_ratio: f64,
    ) -> Self {
        let vfov = vfov.to_radians();
        let h = (vfov / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit_vector();
        let u = Vec3::cross(&vup, &w).unit_vector();
        let v = Vec3::cross(&w, &u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;

        let lower_left_corner = origin - (horizontal + vertical) / 2.0 - w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}
