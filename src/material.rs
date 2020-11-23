use super::hittable::{Face, Hit};
use super::random::{random_in_unit_sphere, random_unit_vector};
use super::vec3::{Color, Ray, Vec3};
use rand::{rngs::SmallRng, Rng};

pub enum ScatterResult {
    Reflect(Color, Ray),
    Emit(Color),
    Absorb,
}

pub trait Scatter {
    fn scatter(&self, rng: &mut SmallRng, ray_in: &Ray, hit: &Hit) -> ScatterResult;
}

#[derive(Copy, Clone)]
pub struct Diffuse {
    pub albedo: Color,
}

impl Scatter for Diffuse {
    fn scatter(&self, rng: &mut SmallRng, _ray_in: &Ray, hit: &Hit) -> ScatterResult {
        let mut scatter_direction: Vec3 = hit.normal + random_unit_vector(rng);

        if scatter_direction.near_zero() {
            scatter_direction = hit.normal.into()
        }

        ScatterResult::Reflect(self.albedo, Ray::new(hit.point, scatter_direction.into()))
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Scatter for Metal {
    fn scatter(&self, rng: &mut SmallRng, ray_in: &Ray, hit: &Hit) -> ScatterResult {
        let reflected = ray_in.direction.reflect(hit.normal);
        ScatterResult::Reflect(
            self.albedo,
            Ray::new(
                hit.point,
                (reflected + self.fuzz * random_in_unit_sphere(rng)).unit_vector(),
            ),
        )
    }
}

#[derive(Copy, Clone)]
pub struct Dielectric {
    pub refractive_index: f64,
}

impl Dielectric {
    fn reflectance(&self, cos_theta: f64) -> f64 {
        let r0 = (1.0 - self.refractive_index) / (1.0 + self.refractive_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cos_theta).powf(5.0)
    }
}

impl Scatter for Dielectric {
    fn scatter(&self, rng: &mut SmallRng, ray_in: &Ray, hit: &Hit) -> ScatterResult {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = match hit.face {
            Face::Front => 1.0 / self.refractive_index,
            Face::Back => self.refractive_index,
        };

        let unit_direction = ray_in.direction;

        let cos_theta = unit_direction.cos_theta(-hit.normal);
        let sin_theta = unit_direction.sin_theta(hit.normal);

        let can_refract = refraction_ratio * sin_theta <= 1.0;
        // let can_refract = true;

        let reflectance = self.reflectance(cos_theta);

        let output_direction = if can_refract && reflectance <= rng.gen::<f64>() {
            unit_direction.refract(hit.normal, refraction_ratio)
        } else {
            unit_direction.reflect(hit.normal)
        };

        ScatterResult::Reflect(
            attenuation,
            Ray::new(hit.point, output_direction.unit_vector()),
        )
    }
}

#[derive(Copy, Clone)]
pub struct Light {
    pub color: Color,
}

impl Scatter for Light {
    fn scatter(&self, _rng: &mut SmallRng, _ray_in: &Ray, _hit: &Hit) -> ScatterResult {
        ScatterResult::Emit(self.color)
    }
}
