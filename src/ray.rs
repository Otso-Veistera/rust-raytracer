// Import the Vec3 module from the same crate
use crate::vec3::Vec3;

// Define a structure to represent rays in three-dimensional space
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub orig: Vec3, // Origin point of the ray
    pub dir: Vec3,  // Direction vector of the ray
}

impl Ray {
    // Constructor to create a new Ray instance
    pub fn new(orig: Vec3, dir: Vec3) -> Self {
        Ray { orig, dir }
    }

    // Getter method to retrieve the origin of the ray
    pub fn origin(&self) -> Vec3 {
        self.orig
    }

    // Getter method to retrieve the direction of the ray
    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    // Compute a point along the ray given a parameter t
    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + self.dir * t // Calculate the point using the parameterized equation of a ray
    }
}
