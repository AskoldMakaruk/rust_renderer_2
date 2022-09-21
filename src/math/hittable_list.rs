use std::rc::Rc;

use super::hit::{Hit, Hittable};

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, r: &super::ray::Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut hit_occured = false;
        let mut closest = t_max.clone();
        let mut rec: Hit;

        for obj in self.objects {
            let hit = obj.as_ref().hit(r, t_min, closest);
            match hit {
                Some(i) => {
                    hit_occured = true;
                    closest = i.t;
                    rec = i
                }
                _ => {}
            }
        }

        match hit_occured {
            true => Option::Some(rec),
            false => Option::None,
        }
    }
}

impl HittableList {
    fn clear(mut self) {
        self.clear();
    }
}
