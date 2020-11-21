use super::random::random_in_unit_disk;
use super::vec3::{CrossProduct, Point3, Ray, UnitVec3, Vec3};
use rand::Rng;
use std::f64::consts::PI;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: UnitVec3,
    v: UnitVec3,
    lens_radius: f64,
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
        aperture: f64,
        focus_distance: f64,
    ) -> Self {
        let vfov = vfov.to_radians();
        let h = (vfov / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;

        let lower_left_corner = origin - (horizontal + vertical) / 2.0 - focus_distance * w;

        let lens_radius = aperture / 2.0;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
        }
    }

    pub fn get_ray<R: Rng>(&self, rng: &mut R, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk(rng);
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            (self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset)
                .unit_vector(),
        )
    }
}
