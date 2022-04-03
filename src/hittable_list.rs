use crate::hittable::{HitRecord, Hittable};
use crate::Ray;
use std::cell::RefCell;
use std::rc::Rc;

pub struct HittableList<'a>(Vec<Rc<RefCell<dyn Hittable + 'a>>>);

impl<'a> HittableList<'a> {
    pub fn new() -> HittableList<'a> {
        HittableList(Vec::new())
    }
    pub fn add(&mut self, hittable: impl Hittable + 'a) {
        self.0.push(Rc::new(RefCell::new(hittable)));
    }
}

impl Hittable for HittableList<'_> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.0 {
            if object
                .borrow()
                .hit(&ray, t_min, closest_so_far, &mut temp_rec)
            {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }
}
