use super::vec3::{Point, Vec3};

#[derive(Clone, Copy)]
pub struct Ray {
    pub orig: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(orig: Point, direction: Vec3) -> Self {
        Self { orig, direction }
    }

    pub fn at(&self, t: f32) -> Point {
        self.orig + self.direction.mul(t)
    }
}
#[cfg(test)]
mod tests {
    use crate::math::vec3::{Point, Vec3};

    use super::Ray;

    #[test]
    fn ray_at() {
        let ray = Ray::new(Point::ZERO, Vec3::new(1.0, 0.0, 0.0));
        let at = ray.at(2.0);

        assert_eq!(at.x, 2.0);
    }
}
