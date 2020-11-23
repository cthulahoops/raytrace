use super::hittable::{Hit, Hittable};
use super::material::Material;
use super::vec3::{Point3, Ray, UnitVec3, Vec3};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: &Material) -> Self {
        Sphere {
            center,
            radius,
            material: material.clone(),
        }
    }

    fn outward_normal(&self, point: Point3) -> UnitVec3 {
        (point - self.center).unit_vector()
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let center = self.center;
        let radius = self.radius;

        let oc = ray.origin - center;
        let half_b = Vec3::dot(oc, ray.direction.into());
        let c = oc.length_squared() - radius * radius;

        let discriminant = half_b * half_b - c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = -half_b - sqrtd;

        if root < t_min || root > t_max {
            root = -half_b + sqrtd;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let point = ray.at(root);

        Some(Hit::new(root, ray, point, self.outward_normal(point)))
    }
}

pub struct World(Vec<Sphere>);

impl World {
    pub fn new(spheres: Vec<Sphere>) -> Self {
        World(spheres)
    }

    pub fn hit<'a>(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<(Hit, &'a Sphere)> {
        let mut closest_so_far = t_max;
        let mut best_so_far: Option<(Hit, &Sphere)> = None;
        for sphere in &self.0 {
            if let Some(hit) = sphere.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                best_so_far = Some((hit, sphere));
            }
        }
        best_so_far
    }
}
