#![allow(unused)]
use rand::random;
use std::{
    fmt::{write, Display},
    ops,
};

use crate::interval::Interval;

#[derive(Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        // *Box::new(Vec3 { x, y, z })
        Vec3 { x, y, z }
    }

    pub fn new_zero() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn unit(&self) -> Vec3 {
        let length = self.length();
        Vec3::new(self.x / length, self.y / length, self.z / length)
    }

    pub fn write_ppm(&self) -> String {
        format!(
            "{} {} {}\n",
            (255.999 * self.x) as u8,
            (255.999 * self.y) as u8,
            (255.999 * self.z) as u8
        )
    }

    pub fn as_tuple(&self) -> [u8; 3] {
        [
            (255.999 * self.x) as u8,
            (255.999 * self.y) as u8,
            (255.999 * self.z) as u8,
        ]
    }

    pub fn as_tuple_processed(&self, sample_per_pixel: u32) -> [u8; 3] {
        let linear_to_gamma = |x: f64| x.sqrt();

        let (mut r, mut g, mut b) = (self.x, self.y, self.z);

        let scale = 1.0 / sample_per_pixel as f64;
        r *= scale;
        g *= scale;
        b *= scale;

        r = linear_to_gamma(r);
        g = linear_to_gamma(g);
        b = linear_to_gamma(b);

        let intv = Interval::new(0.0, 0.999);
        [
            (256.0 * intv.clamp(r)) as u8,
            (256.0 * intv.clamp(g)) as u8,
            (256.0 * intv.clamp(b)) as u8,
        ]
    }

    pub fn random_vec() -> Self {
        Vec3::new(random::<f64>(), random::<f64>(), random::<f64>())
    }

    pub fn random(min: f64, max: f64) -> Self {
        Vec3::new(
            random::<f64>() * (max - min) + min,
            random::<f64>() * (max - min) + min,
            random::<f64>() * (max - min) + min,
        )
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Box::new(Vec3::random(-1.0, 1.0));
            if p.length_squared() < 1.0 {
                return *p;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Vec3::random_in_unit_sphere().unit()
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_in_unit_sphere();
        if normal.dot(&on_unit_sphere) > 0.0 {
            on_unit_sphere
        } else {
            on_unit_sphere * -1.0
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let random_float = || {
            random::<f64>() * 2.0 - 1.0
        };
        loop {
            let p = Vec3::new(random_float(), random_float(), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        *self - *normal * self.dot(normal) * 2.0
    }

    pub fn refract(&self, normal: &Vec3, eta_ratio: f64) -> Vec3 {
        let cos_theta = (*self * -1.0).dot(normal).min(1.0);
        let r_out_perp = (*self + *normal * cos_theta) * eta_ratio;
        let r_out_parallel = *normal * (-(1.0 - r_out_perp.length_squared()).abs().sqrt());
        r_out_perp + r_out_parallel
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
    }
}

impl ops::Index<isize> for Vec3 {
    type Output = f64;
    fn index(&self, index: isize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl ops::IndexMut<isize> for Vec3 {
    fn index_mut(&mut self, index: isize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Self;
    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl std::clone::Clone for Vec3 {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Vec3 {}
