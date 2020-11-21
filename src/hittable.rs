use super::vec3::{Point3, Ray, UnitVec3};

pub enum Face {
    Front,
    Back,
}

pub struct Hit {
    pub point: Point3,
    pub normal: UnitVec3,
    pub t: f64,
    pub face: Face,
}

impl Hit {
    pub fn new(root: f64, ray: &Ray, point: Point3, outward_normal: UnitVec3) -> Self {
        let face = if UnitVec3::cos_theta(ray.direction, outward_normal) < 0.0 {
            Face::Front
        } else {
            Face::Back
        };
        Hit {
            t: root,
            point: point,
            normal: match face {
                Face::Front => outward_normal,
                Face::Back => -outward_normal,
            },
            face: face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}
