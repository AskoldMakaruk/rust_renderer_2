use std::ops::{Add, Mul, Neg, Sub};

pub type Point = Vec3;
pub type Color = Vec3;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Color {
    pub fn write_color(&self) -> String {
        (self.x as i32).to_string()
            + " "
            + &(self.y as i32).to_string()
            + " "
            + &(self.z as i32).to_string()
    }
}
impl Vec3 {
    pub const ZERO: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    pub const fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn mul(&self, t: f32) -> Self {
        Vec3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
    pub fn div(&self, t: f32) -> Self {
        self.mul(1.0 / t)
    }

    pub fn length(self) -> f32 {
        f32::sqrt(self.length_squared())
    }

    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unit_vec(&self) -> Self {
        self.div(self.length())
    }
}
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
impl ToString for Vec3 {
    fn to_string(&self) -> String {
        self.x.to_string() + " " + &self.y.to_string() + " " + &self.z.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::math::vec3::Vec3;
    #[test]
    fn vec3_dot() {
        let vec1 = Vec3::new(1.0, 3.0, -5.0);
        let vec2 = Vec3::new(4.0, -2.0, -1.0);

        assert_eq!(vec1.dot(vec2), 3.0);
    }
    #[test]
    fn vec3_dot_self() {
        let vec1 = Vec3::new(1.0, 3.0, -5.0);

        assert_eq!(vec1.clone().dot(vec1), 35.0);
    }

    #[test]
    fn vec3_add() {
        let vec = Vec3::new(1.0, 1.0, 1.0);
        let vec2 = vec.clone();

        let vec = vec + vec2;

        assert_eq!(vec, Vec3::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn vec3_to_string() {
        let vec = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(vec.to_string(), "1 1 1")
    }
}
