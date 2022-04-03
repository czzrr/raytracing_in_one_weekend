use crate::hittable::Hittable;
use crate::vec3::Point3;
use crate::HitRecord;
use crate::Ray;

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    /* The ray P(t) = A + tb hits the sphere with center C and radius r if
       (P(t) - C) . (P(t) - C) = r^2
       for any t. Expanding:
       (A + tb - C) * (A + tb - C) = r^2
       <=>
       t^2 * b.b + 2tb . (A - C) + (A - C) . (A - C) - r^2 = 0
       This is a quadratic equation in t.
       Returns the smallest (positive) value of t.
    */
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        // A - C in above
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();
        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        rec.t = root;
        rec.p = ray.at(rec.t);
        // Outward normal vector to surface of sphere at intersection point with ray
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(&ray, &outward_normal);

        true
    }
}
