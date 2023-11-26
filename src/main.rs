// Importing necessary libraries
use minifb::{Key, Window, WindowOptions};
use std::convert::TryInto;

// Struct representing a 3D vector
#[derive(Clone, Copy)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    // Constructor for Vec3
    fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    // Returns the unit vector of the current vector
    fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    // Returns the length of the vector
    fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    // Returns the squared length of the vector
    fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    // Returns the dot product of the vector with another vector
    fn dot(&self, rhs: Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

// Overloading operators for Vec3
impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3 {
            x: self * vec.x,
            y: self * vec.y,
            z: self * vec.z,
        }
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        (1.0 / t) * self
    }
}

// Struct representing a Ray in 3D space
#[derive(Clone, Copy)]
struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    // Constructor for Ray
    fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    // Returns the point along the ray at a given parameter t
    fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}

// Struct representing a Sphere in 3D space
#[derive(Clone, Copy)]
struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    // Checks if the given ray intersects with the sphere, returns the intersection parameter if it does
    fn hit(&self, ray: &Ray) -> Option<f64> {
        let oc = ray.origin - self.center;
        let a = ray.direction.squared_length();
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            // Ray hits the sphere
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            if t1 > 0.0 || t2 > 0.0 {
                // Return the minimum positive solution
                Some(t1.min(t2))
            } else {
                // Both solutions are negative
                None
            }
        } else {
            // Ray does not hit the sphere
            None
        }
    }
}

// Finds the closest intersection between a ray and a list of spheres
fn hit_sphere<'a>(world: &'a [Sphere], ray: &'a Ray) -> Option<(f64, &'a Sphere)> {
    let mut closest_t = f64::INFINITY;
    let mut hit_sphere: Option<&'a Sphere> = None;

    for sphere in world {
        if let Some(t) = sphere.hit(ray) {
            if t < closest_t {
                closest_t = t;
                hit_sphere = Some(sphere);
            }
        }
    }

    hit_sphere.map(|sphere| (closest_t, sphere))
}

// Computes the color of a ray, taking into account intersections with spheres in the world
fn ray_color(ray: &Ray, world: &[Sphere]) -> Vec3 {
    if let Some((t, sphere)) = hit_sphere(world, ray) {
        let hit_point = ray.at(t);
        let normal = Vec3::unit_vector(&(hit_point - sphere.center));
        return 0.5 * (normal + Vec3::new(1.0, 1.0, 1.0));
    }

    // Background color if no intersection
    Vec3::new(0.0, 0.0, 0.0)
}

fn main() {
    // Image dimensions
    let image_width = 800;
    let image_height = 400;

    // Creating a window using the minifb library
    let mut window = Window::new(
        "Ray Tracing",
        image_width.try_into().unwrap(),
        image_height.try_into().unwrap(),
        WindowOptions {
            scale: minifb::Scale::X1, // Set zoom to 1x
            ..WindowOptions::default()
        },
    )
    .expect("Failed to create window");

    // Buffer to store pixel values for the window
    let mut buffer: Vec<u32> = vec![0; image_width * image_height];

    // Camera and scene setup
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    // List of spheres in the scene
    let world = vec![
        Sphere {
            center: Vec3::new(0.0, 0.0, -1.0),
            radius: 0.5,
        },
        Sphere {
            center: Vec3::new(0.0, -100.5, -1.0),
            radius: 100.0,
        },
    ];

    // Rendering loop
    while window.is_open() {
        for j in 0..image_height {
            for i in 0..image_width {
                let index = j * image_width + i;
                let u = i as f64 / (image_width - 1) as f64;
                let v = j as f64 / (image_height - 1) as f64;

                // Generate a ray for the current pixel
                let ray = Ray::new(
                    origin,
                    lower_left_corner + u * horizontal + v * vertical - origin,
                );

                // Compute the color of the ray and store it in the buffer
                let color = ray_color(&ray, &world);

                // Convert color components to u8 and store in the buffer
                let ir = (255.999 * color.x) as u8;
                let ig = (255.999 * color.y) as u8;
                let ib = (255.999 * color.z) as u8;

                buffer[index] = (ir as u32) << 16 | (ig as u32) << 8 | ib as u32;
            }
        }

        // Update the window with the buffer
        window
            .update_with_buffer(
                &buffer,
                image_width.try_into().unwrap(),
                image_height.try_into().unwrap(),
            )
            .unwrap();

        // Check for key presses (e.g., Esc key to exit)
        if window.is_key_down(Key::Escape) {
            break;
        }
    }
}
