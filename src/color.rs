// Import necessary modules and structs
use std::fmt;
use core::ops::{Mul, Add};

// Define a structure to represent colors with RGB components
#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: u8, // Red component
    pub g: u8, // Green component
    pub b: u8, // Blue component
}

impl Color {
    // Constructor to create a new Color instance from floating-point values
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color {
            r: (255.999 * r) as u8, // Convert and clamp the red component
            g: (255.999 * g) as u8, // Convert and clamp the green component
            b: (255.999 * b) as u8, // Convert and clamp the blue component
        }
    }
}

// Implement the Display trait for pretty-printing Color instances
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

// Implement operator overloads for multiplication with scalar values
impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, scalar: f64) -> Color {
        Color {
            r: (self.r as f64 * scalar) as u8, // Multiply and convert the red component
            g: (self.g as f64 * scalar) as u8, // Multiply and convert the green component
            b: (self.b as f64 * scalar) as u8, // Multiply and convert the blue component
        }
    }
}

// Implement operator overloads for multiplication with scalar values (reverse order)
impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, color: Color) -> Color {
        Color {
            r: (color.r as f64 * self) as u8, // Multiply and convert the red component
            g: (color.g as f64 * self) as u8, // Multiply and convert the green component
            b: (color.b as f64 * self) as u8, // Multiply and convert the blue component
        }
    }
}

// Implement operator overloads for addition of two Color instances
impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            r: self.r.saturating_add(other.r), // Add red components with saturation
            g: self.g.saturating_add(other.g), // Add green components with saturation
            b: self.b.saturating_add(other.b), // Add blue components with saturation
        }
    }
}

// Function to write a Color instance to a given output stream
