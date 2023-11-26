// Import necessary structs and traits from other modules
use crate::ray::Ray;
use crate::vec3::Vec3;

// Define a structure to store information about a hit point
#[derive(Debug, Copy, Clone, Default)]
pub struct HitRecord {
    pub p: Vec3,         // Hit point in 3D space
    pub normal: Vec3,    // Surface normal at the hit point
    pub t: f64,          // Parameter along the ray at the hit point
    pub front_face: bool, // Indicates if the ray hit the front or back face of the object
}

// Define a trait for hittable objects
pub trait Hittable {
    // Method to check for ray-object intersections
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
