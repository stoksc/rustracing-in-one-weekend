use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;
use crate::vec3::{self, Vec3};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(c: Vec3, r: f32) -> Sphere {
        Sphere {
            center: c,
            radius: r,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = ray.origin() - &self.center;
        let a = vec3::dot(ray.direction(), ray.direction());
        let b = 2.0 * vec3::dot(&oc, ray.direction());
        let c = vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let temp = (-b - (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = ray.point_at_parameter(temp);
                rec.normal = (rec.p.clone() - self.center.clone()) / self.radius;
                return true;
            }
            let temp = (-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = ray.point_at_parameter(temp);
                rec.normal = (rec.p.clone() - self.center.clone()) / self.radius;
                return true;
            }
        }
        return false;
    }
}
