use super::{
    hit::{Hit, Hittable},
    ray::Ray,
    vec3::{Point, Vec3},
};

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let oc: Vec3 = r.orig - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(&r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return Option::None;
        }
        let sqrtd = (-half_b - discriminant.sqrt()) / (a);

        let root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return Option::None;
            }
        }
        let point = r.at(root);
        let outward_normal = (point - self.center).div(self.radius);
        Option::Some(Hit::new(point, root, &outward_normal, r))
    }
}
