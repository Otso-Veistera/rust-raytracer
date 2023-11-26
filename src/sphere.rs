// Import necessary structs and traits from other modules
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

// Define a structure to represent a sphere in 3D space
pub struct Sphere {
    center: Vec3, // Center of the sphere
    radius: f64,  // Radius of the sphere
}

impl Sphere {
    // Constructor for creating a new Sphere instance
    pub fn new(center: Vec3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

// Implement the Hittable trait for Sphere
impl Hittable for Sphere {
    // Method to check for ray-sphere intersections
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        // Calculate necessary parameters for the quadratic equation
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        // Check if the ray intersects the sphere
        if discriminant > 0.0 {
            let sqrtd = discriminant.sqrt();

            // Check the two possible roots of the quadratic equation
            let mut root = (-half_b - sqrtd) / a;
            if root < t_min || t_max < root {
                root = (-half_b + sqrtd) / a;
                if root < t_min || t_max < root {
                    return false;
                }
            }

            // Update the hit record with intersection information
            rec.t = root;
            rec.p = r.at(rec.t);
            rec.normal = (rec.p - self.center) / self.radius;

            return true;
        }

        false
    }
}
