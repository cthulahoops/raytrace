use super::vec3::{UnitVec3, Vec3};
use rand::Rng;

pub fn random_in_unit_disk<T: Rng>(rng: &mut T) -> Vec3 {
    loop {
        let p = Vec3::new(
            2.0 * rng.gen::<f64>() - 1.0,
            2.0 * rng.gen::<f64>() - 1.0,
            0.0,
        );
        if p.length_squared() > 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_in_unit_sphere<T: Rng>(rng: &mut T) -> Vec3 {
    loop {
        let p = random_vec3(rng);
        if p.length_squared() > 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector<T: Rng>(rng: &mut T) -> UnitVec3 {
    random_in_unit_sphere(rng).unit_vector()
}

pub fn random_vec3<T: Rng>(rng: &mut T) -> Vec3 {
    Vec3::new(
        2.0 * rng.gen::<f64>() - 1.0,
        2.0 * rng.gen::<f64>() - 1.0,
        2.0 * rng.gen::<f64>() - 1.0,
    )
}

pub fn random_vec3_range<R: Rng>(rng: &mut R, min: f64, max: f64) -> Vec3 {
    Vec3::new(
        random_f64_range(rng, min, max),
        random_f64_range(rng, min, max),
        random_f64_range(rng, min, max),
    )
}

pub fn random_f64_range<R: Rng>(rng: &mut R, min: f64, max: f64) -> f64 {
    rng.gen_range(min, max)
}
