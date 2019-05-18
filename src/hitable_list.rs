use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;

pub struct HitableList {
    pub list: Vec<Box<Hitable>>,
}

impl HitableList {
    pub fn new() -> HitableList {
        HitableList { list: Vec::new() }
    }
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for h in self.list.iter() {
            if h.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.update(&temp_rec);
            }
        }
        hit_anything
    }
}
