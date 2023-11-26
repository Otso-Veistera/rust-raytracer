// Import necessary modules and libraries
use std::io;
use minifb::{Key, Window, WindowOptions, MouseButton, MouseMode};

// Import custom modules and types
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use color::Color;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

// Function to calculate the color of a ray in the scene
fn ray_color(r: &Ray, world: &impl Hittable) -> Color {
    // Initialize a hit record with default values
    let mut rec = HitRecord::default();

    // Check if the ray hits any object in the scene
    if world.hit(r, 0.0, f64::INFINITY, &mut rec) {
        // Calculate the color based on the surface normal
        0.5 * Color::new(rec.normal.x + 1.0, rec.normal.y + 1.0, rec.normal.z + 1.0)
    } else {
        // If the ray doesn't hit any object, calculate background color based on ray direction
        let unit_direction = r.direction().normalized();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

// Main function
fn main() -> io::Result<()> {
    // Image dimensions and aspect ratio
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // Create a window using the minifb library
    let mut window = Window::new(
        "Ray Tracing",
        image_width.try_into().unwrap(),
        image_height.try_into().unwrap(),
        WindowOptions {
            scale: minifb::Scale::X1,  // Set the window scale to 1x
            ..WindowOptions::default()
        },
    )
        .expect("Failed to create window");

    // Buffer to store pixel values for the window
    let mut buffer: Vec<u32> = vec![0; image_width as usize * image_height as usize];

    // Camera and scene setup
    let aspect_ratio = image_width as f64 / image_height as f64;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;

    let mut pitch: f64 = 1.0;
    let mut yaw: f64 = 5.0;
    let mut focal_length = 1.0;

    let mut origin = Vec3::new(0.0, 0.0, 0.0);

    // World setup with a list of spheres
    let world = HittableList::new(vec![
        Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 0.5)),
        Box::new(Sphere::new(Vec3::new(1.0, 1.0, 0.0), 0.4)),  // Ground sphere
        // Add other objects as needed
    ]);

    // Camera speed and rotation speed
    let camera_speed = 0.1;
    let rotation_speed = 0.02;

    // Mouse input variables
    let mut prev_mouse_pos = (0.0, 0.0);
    let mut mouse_pressed = false;

    // Rendering loop
    while window.is_open() {
        // Handle camera movement using arrow keys
        if window.is_key_down(Key::W) {
            origin.x += camera_speed;
        }
        if window.is_key_down(Key::S) {
            origin.x -= camera_speed;
        }
        if window.is_key_down(Key::A) {
            origin.z -= camera_speed;
        }
        if window.is_key_down(Key::D) {
            origin.z += camera_speed;
        }

        // Adjust focal length using keys
        if window.is_key_down(Key::E) {
            origin.y += camera_speed;
        }
        if window.is_key_down(Key::Q) {
            origin.y -= camera_speed;
        }

        // Handle camera rotation using mouse
        if window.get_mouse_down(MouseButton::Left) {
            let mouse_pos = window.get_mouse_pos(MouseMode::Pass).unwrap_or((0.0, 0.0));

            if !mouse_pressed {
                prev_mouse_pos = mouse_pos;
                mouse_pressed = true;
            }

            let delta_x = mouse_pos.0 - prev_mouse_pos.0;
            let delta_y = mouse_pos.1 - prev_mouse_pos.1;

            yaw += rotation_speed as f64 * delta_x as f64;
            pitch += rotation_speed as f64 * delta_y as f64;

            // Clamp pitch to avoid flipping
            pitch = pitch.clamp(-std::f64::consts::FRAC_PI_2, std::f64::consts::FRAC_PI_2);
            yaw = yaw.clamp(0.0, 2.0 * std::f64::consts::PI);

            prev_mouse_pos = mouse_pos;
        } else {
            mouse_pressed = false;
        }

        // Handle zooming using mouse scroll
        if let Some((_scroll_x, scroll_y)) = window.get_scroll_wheel() {
            focal_length -= scroll_y as f64 * 0.1;
        }

        // Calculate the new direction based on pitch and yaw
        let direction = Vec3::new(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos(),
        )
            .normalized();

        // Update horizontal and vertical vectors based on the new direction
        let right = direction.cross(Vec3::new(0.0, 1.0, 0.0)).normalized();
        let up = right.cross(direction).normalized();

        // Update the camera's horizontal and vertical vectors
        let horizontal = right * viewport_width;
        let vertical = up * viewport_height;

        // Rendering
        for j in 0..image_height {
            for i in 0..image_width {
                let index = j as usize * image_width as usize + i as usize;

                let u = i as f64 / (image_width - 1) as f64;
                let v = j as f64 / (image_height - 1) as f64;

                let ray = Ray::new(
                    origin,
                    horizontal * u + vertical * v - Vec3::new(0.0, 0.0, focal_length),
                );

                // Calculate color for the pixel using the ray
                let color = ray_color(&ray, &world);

                // Convert color components to u32 values
                let ir = color.r as u32;
                let ig = color.g as u32;
                let ib = color.b as u32;

                // Combine color components into a single u32 value and store in the buffer
                buffer[index] = (ir << 16) | (ig << 8) | ib;
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

    Ok(())
}
