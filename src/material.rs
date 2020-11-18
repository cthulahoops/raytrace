use super::vec3::{Ray, Color, Vec3, Point3};
use rand::Rng;


fn random_vector<T: Rng>(rng: &mut T) -> Vec3 {
    Vec3::new(2.0 * rng.gen::<f64>() - 1.0, 2.0 * rng.gen::<f64>() - 1.0, rng.gen::<f64>() - 1.0)
}

fn random_in_unit_sphere<T: Rng>(rng: &mut T) -> Vec3 {
    loop {
        let p = random_vector(rng);
        if p.length_squared() > 1.0 {
            continue;
        }
        return p
    }
}

fn random_unit_vector<T: Rng>(rng: &mut T) -> Vec3 {
    random_in_unit_sphere(rng).unit_vector()
}

pub enum Face {
    Front,
    Back,
}

pub struct Hit {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub face: Face,
}

impl Hit {
    pub fn new(root: f64, ray: &Ray, sphere: &Sphere) -> Self {
        let point = ray.at(root);

        let outward_normal = (point - sphere.center) / sphere.radius;
        let face = if Vec3::dot(ray.direction, outward_normal) < 0.0 {
            Face::Front
        } else {
            Face::Back
        };
        Hit {
            t: root,
            point: point,
            normal: match face {
                Face::Front => outward_normal,
                Face::Back => -1.0 * outward_normal,
            },
            face: face,
        }
    }
}

// pub trait Material {
//     fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Color, Ray)>;
// }

#[derive(Copy, Clone)]
pub struct Lambertian {
	pub albedo: Color
}

impl Lambertian {
	pub fn scatter<R: Rng>(&self, rng: &mut R, _ray_in: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
		let mut scatter_direction = hit.normal + random_unit_vector(rng);

		if scatter_direction.near_zero() {
			scatter_direction = hit.normal
		}
	
		Some((self.albedo, Ray::new( hit.point, scatter_direction )))
	}
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Lambertian
}

impl Sphere {
    pub fn new(center : Point3, radius: f64, material: Lambertian) -> Self {
        Sphere { center, radius, material }
    }
}
