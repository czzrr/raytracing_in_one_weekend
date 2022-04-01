use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    // pub fn neg(self) -> Vec3 {
    //     let mut v = self.clone();
    //     v.neg_mut();
    //     v
    // }

    // pub fn neg_mut(&mut self) {
    //     self.x = -self.x;
    //     self.y = -self.y;
    //     self.z = -self.z;
    // }

    // pub fn add(&self, other: &Vec3) -> Vec3 {
    //     let mut v = self.clone();
    //     v.add_mut(other);
    //     v
    // }

    // pub fn add_mut(&mut self, other: &Vec3) {
    //     self.x = self.x + other.x;
    //     self.y = self.y + other.y;
    //     self.z = self.z + other.z;
    // }

    // pub fn mul_scalar(&self, t: f64) -> Vec3 {
    //     let mut v = self.clone();
    //     v.mul_scalar_mut(t);
    //     v
    // }

    // pub fn mul_scalar_mut(&mut self, t: f64) {
    //     self.x = self.x * t;
    //     self.y = self.y * t;
    //     self.z = self.z * t;
    // }

    // pub fn div(&self, t: f64) -> Vec3 {
    //     let mut v = self.clone();
    //     v.div_mut(t);
    //     v
    // }

    // pub fn div_mut(&mut self, t: f64) {
    //     self.mul_scalar(1.0 / t);
    // }

    // pub fn mul_vec(&self, other: &Vec3) -> Vec3 {
    //     let x = self.x * other.x;
    //     let y = self.y * other.y;
    //     let z = self.z * other.z;
    //     Vec3 { x, y, z }
    // }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;
        Vec3 { x, y, z }
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

pub fn write_color(pixel_color: Color) {
    println!(
        "{} {} {}",
        (255.999 * pixel_color.x) as i32,
        (255.999 * pixel_color.y) as i32,
        (255.999 * pixel_color.z) as i32
    );
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let x = -self.x;
        let y = -self.y;
        let z = -self.z;
        Vec3 { x, y, z }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        Vec3 { x, y, z }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;
        Vec3 { x, y, z }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, t: f64) -> Self::Output {
        let x = self.x * t;
        let y = self.y * t;
        let z = self.z * t;
        Vec3 { x, y, z }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Self::Output {
        self * (1.0 / t)
    }
}
