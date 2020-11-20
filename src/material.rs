use super::hittable::{Face, Hit};
use super::vec3::{Color, Ray};
use rand::{rngs::SmallRng, Rng};
use super::random::{random_unit_vector, random_in_unit_sphere};

pub trait Scatter {
    fn scatter(&self, rng: &mut SmallRng, ray_in: &Ray, hit: &Hit) -> Option<(Color, Ray)>;
}

#[derive(Copy, Clone)]
pub struct Diffuse {
    pub albedo: Color,
}

impl Scatter for Diffuse {
    fn scatter(&self, rng: &mut SmallRng, _ray_in: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        let mut scatter_direction = hit.normal + random_unit_vector(rng);

        if scatter_direction.near_zero() {
            scatter_direction = hit.normal
        }

        Some((self.albedo, Ray::new(hit.point, scatter_direction)))
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Scatter for Metal {
    fn scatter(&self, rng: &mut SmallRng, ray_in: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        let reflected = ray_in.direction.unit_vector().reflect(hit.normal);
        Some((
            self.albedo,
            Ray::new(
                hit.point,
                reflected + self.fuzz * random_in_unit_sphere(rng),
            ),
        ))
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
    fn scatter(&self, rng: &mut SmallRng, ray_in: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = match hit.face {
            Face::Front => 1.0 / self.refractive_index,
            Face::Back => self.refractive_index,
        };

        let unit_direction = ray_in.direction.unit_vector();

        let cos_theta = unit_direction.cos_theta(hit.normal);
        let sin_theta = unit_direction.sin_theta(hit.normal);

        let can_refract = refraction_ratio * sin_theta <= 1.0;
        // let can_refract = true;

        let reflectance = self.reflectance(cos_theta);

        let output_direction = if can_refract && reflectance <= rng.gen::<f64>() {
            unit_direction.refract(hit.normal, refraction_ratio)
        } else {
            unit_direction.reflect(hit.normal)
        };

        Some((attenuation, Ray::new(hit.point, output_direction)))
    }
}
