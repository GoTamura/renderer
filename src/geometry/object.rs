use crate::geometry::Hit;
use crate::geometry::Normal3f;
use crate::geometry::Point3f;
use crate::geometry::Ray;
use crate::geometry::ShadeRec;
use crate::geometry::Vector3f;

pub struct Plane {
    point: Point3f,
    normal: Normal3f,
    kepsilon: f64,
}

impl Hit for Plane {
    fn hit(&self, ray: &Ray, tmin: &mut f64, sr: &mut ShadeRec) -> bool {
        let normal: Vector3f = self.normal.into();
        let t = (self.point - ray.o).dot(normal) / ray.d.dot(normal);

        if t > self.kepsilon {
            *tmin = t;
            sr.normal = self.normal;
            sr.local_hit_point = ray.o + ray.d * t;
            true
        } else {
            false
        }
    }
}

pub struct Sphere {
    center: Point3f,
    radius: f64,
    kepsilon: f64,
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, tmin: &mut f64, sr: &mut ShadeRec) -> bool {
        let temp = ray.o - self.center;
        let a = ray.d.dot(ray.d);
        let b = temp.dot(ray.d) * 2.;
        let c = temp.dot(temp) - self.radius * self.radius;
        let disc = b * b - 4.0 * a * c;
        if disc < 0.0 {
            return false;
        } else {
            let e = disc.sqrt();
            let denom = a * 2.;
            let t = (-b - e) / denom;

            if t > self.kepsilon {
                *tmin = t;
                sr.normal = Normal3f::from((temp + ray.d * t) / self.radius);
                sr.local_hit_point = ray.o + ray.d * t;
                return true;
            }
            let t = (-b + e) / denom;
            if t > self.kepsilon {
                *tmin = t;
                sr.normal = Normal3f::from((temp + ray.d * t) / self.radius);
                sr.local_hit_point = ray.o + ray.d * t;
                return true;
            }
        }
        false
    }
}
