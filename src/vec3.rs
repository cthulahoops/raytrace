use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, Clone, Add, Sub, Neg, Mul, Div)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Color = Vec3;
pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unit_vector(self) -> UnitVec3 {
        self.into()
    }

    pub fn near_zero(&self) -> bool {
        const DELTA: f64 = 1e-8;
        self.x.abs() < DELTA && self.y.abs() < DELTA && self.z.abs() < DELTA
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

#[derive(Copy, Clone, Neg)]
pub struct UnitVec3(Vec3);

impl UnitVec3 {
    fn dot(self, rhs: Self) -> f64 {
        self.0.x * rhs.0.x + self.0.y * rhs.0.y + self.0.z * rhs.0.z
    }

    pub fn cos_theta(self, rhs: Self) -> f64 {
        UnitVec3::dot(self, rhs)
    }

    pub fn sin_theta(self, rhs: UnitVec3) -> f64 {
        let cos_theta = f64::min(self.cos_theta(rhs), 1.0);
        (1.0 - cos_theta * cos_theta).sqrt()
    }

    pub fn reflect(self, normal: Self) -> Vec3 {
        self - 2.0 * self.dot(normal) * normal
    }

    pub fn refract(self, normal: Self, refraction_ratio: f64) -> Vec3 {
        let cos_theta = self.cos_theta(normal);
        let r_out_perp = refraction_ratio * (self - cos_theta * normal);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * normal;
        r_out_perp + r_out_parallel
    }
}

impl From<Vec3> for UnitVec3 {
    fn from(vec: Vec3) -> Self {
        UnitVec3(vec / vec.length())
    }
}

impl From<UnitVec3> for Vec3 {
    fn from(vec: UnitVec3) -> Self {
        vec.0
    }
}

impl Add<Vec3> for UnitVec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        self.0 + rhs
    }
}

impl Add<UnitVec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: UnitVec3) -> Self::Output {
        self + rhs.0
    }
}

impl Add<UnitVec3> for UnitVec3 {
    type Output = Vec3;

    fn add(self, rhs: UnitVec3) -> Self::Output {
        self.0 + rhs.0
    }
}

impl Sub<UnitVec3> for UnitVec3 {
    type Output = Vec3;

    fn sub(self, rhs: UnitVec3) -> Self::Output {
        self.0 - rhs.0
    }
}

impl Sub<UnitVec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: UnitVec3) -> Self::Output {
        self - rhs.0
    }
}

impl Sub<Vec3> for UnitVec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        self.0 - rhs
    }
}

impl Mul<f64> for UnitVec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        self.0 * rhs
    }
}

impl Mul<UnitVec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: UnitVec3) -> Self::Output {
        rhs * self
    }
}

pub struct Ray {
    pub origin: Point3,
    pub direction: UnitVec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: UnitVec3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}

pub trait CrossProduct<A> {
    type Output;

    fn cross(self, rhs: A) -> Self::Output;
}

impl CrossProduct<Vec3> for Vec3 {
    type Output = Self;
    fn cross(self, rhs: Self) -> Self {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl CrossProduct<UnitVec3> for UnitVec3 {
    type Output = Self;
    fn cross(self, rhs: Self) -> Self {
        self.0.cross(rhs.0).into()
    }
}

impl CrossProduct<UnitVec3> for Vec3 {
    type Output = Self;
    fn cross(self, rhs: UnitVec3) -> Vec3 {
        self.cross(rhs.0)
    }
}

impl CrossProduct<Vec3> for UnitVec3 {
    type Output = Vec3;
    fn cross(self, rhs: Vec3) -> Vec3 {
        self.0.cross(rhs)
    }
}
