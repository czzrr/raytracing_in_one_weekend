use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::util;

/* A 3D vector of doubles */
#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/* A point in 3D */
pub type Point3 = Vec3;

/* A RGB color */
pub type Color = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;
        Vec3 { x, y, z }
    }

    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }

    pub fn random_unit() -> Vec3 {
        Vec3::new(rand::random::<f64>(), rand::random::<f64>(), rand::random::<f64>())
    }

    pub fn random(min: f64, max: f64) -> Vec3 {
        Vec3::new(util::random_double(min, max), util::random_double(min, max), util::random_double(min, max))
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Self::random_in_unit_sphere().unit_vector()
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

/* Operator overloading for Vec3 */

macro_rules! impl_neg_vec3 {
    ($t:ty) => {
        impl Neg for $t {
            type Output = Vec3;

            fn neg(self) -> Self::Output {
                let x = -self.x;
                let y = -self.y;
                let z = -self.z;
                Vec3 { x, y, z }
            }
        }
    };
}

impl_neg_vec3![Vec3];
impl_neg_vec3![&Vec3];

macro_rules! impl_add_vec3 {
    ($t:ty, $u:ty) => {
        impl Add<$u> for $t {
            type Output = Vec3;

            fn add(self, rhs: $u) -> Self::Output {
                let x = self.x + rhs.x;
                let y = self.y + rhs.y;
                let z = self.z + rhs.z;
                Vec3 { x, y, z }
            }
        }
    };
}

impl_add_vec3![Vec3, Vec3];
impl_add_vec3![Vec3, &Vec3];
impl_add_vec3![&Vec3, Vec3];
impl_add_vec3![&Vec3, &Vec3];

macro_rules! impl_sub_vec3 {
    ($t:ty, $u:ty) => {
        impl Sub<$u> for $t {
            type Output = Vec3;

            fn sub(self, rhs: $u) -> Self::Output {
                let x = self.x - rhs.x;
                let y = self.y - rhs.y;
                let z = self.z - rhs.z;
                Vec3 { x, y, z }
            }
        }
    };
}

impl_sub_vec3![Vec3, Vec3];
impl_sub_vec3![Vec3, &Vec3];
impl_sub_vec3![&Vec3, Vec3];
impl_sub_vec3![&Vec3, &Vec3];

macro_rules! impl_mul_vec3 {
    ($t:ty, $u:ty) => {
        impl Mul<$u> for $t {
            type Output = Vec3;

            fn mul(self, t: $u) -> Self::Output {
                let x = self.x * t;
                let y = self.y * t;
                let z = self.z * t;
                Vec3 { x, y, z }
            }
        }
    };
}

impl_mul_vec3![Vec3, f64];
impl_mul_vec3![&Vec3, f64];

macro_rules! impl_div_vec3 {
    ($t:ty, $u:ty) => {
        impl Div<$u> for $t {
            type Output = Vec3;

            fn div(self, t: $u) -> Self::Output {
                self * (1.0 / t)
            }
        }
    };
}

impl_div_vec3![Vec3, f64];
impl_div_vec3![&Vec3, f64];
