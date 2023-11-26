// Import necessary traits and structs from other modules
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

// Define a struct for a list of hittable objects
pub struct HittableList {
    // Store hittable objects in a vector of trait objects
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    // Constructor for a new HittableList with a vector of hittable objects
    pub fn new(vec1: Vec<Box<dyn Hittable>>) -> Self {
        HittableList { objects: vec1 }
    }

    // Clear the list of hittable objects

}

// Implement the Hittable trait for HittableList
impl Hittable for HittableList {
    // Check for ray-object intersections in the list
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        // Initialize variables to track intersection information
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        // Iterate over each object in the list
        for object in &self.objects {
            // Create a temporary HitRecord to store intersection information
            let mut temp_rec = HitRecord::default();

            // Check for intersection with the current object
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                // Update information if an intersection is found
                hit_anything = true;
                closest_so_far = temp_rec.t;

                // Update the main hit record with the new information
                rec.t = temp_rec.t;
                rec.p = temp_rec.p;
                rec.normal = temp_rec.normal;
            }
        }

        // Return whether any intersection was found
        hit_anything
    }
}
