use std::ops::Neg;

use super::{
    ray::Ray,
    vec3::{Point, Vec3},
};

pub struct Hit {
    pub p: Point,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}

impl Hit {
    pub fn new(p: Point, t: f32, outward_normal: &Vec3, r: &Ray) -> Self {
        let (normal, front_face) = Hit::get_face_normal(r, outward_normal);
        Self {
            p,
            normal,
            t,
            front_face,
        }
    }

    fn get_face_normal(r: &Ray, outward_normal: &Vec3) -> (Vec3, bool) {
        let front_face = r.direction.dot(outward_normal) < 0.0;
        match front_face {
            true => (outward_normal.clone(), true),
            false => (outward_normal.neg(), false),
        }
    }
}
