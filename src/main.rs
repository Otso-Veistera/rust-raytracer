use image::{ImageBuffer, Rgba};
use std::f32::consts::FRAC_PI_4;

const WIDTH: u32 = 80;
const HEIGHT: u32 = 60;

#[derive(Clone, Copy)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

struct Triangle {
    vertices: [Vec3; 3],
    color: Rgba<u8>,
}

impl Triangle {
    fn new(v1: Vec3, v2: Vec3, v3: Vec3, color: Rgba<u8>) -> Triangle {
        Triangle {
            vertices: [v1, v2, v3],
            color,
        }
    }
}

fn main() {
    let mut img = ImageBuffer::new(WIDTH, HEIGHT);

    // Define a pyramid with larger coordinates
    let pyramid = vec![
        // Base (blue)
        Triangle::new(
            Vec3 { x: -1.0, y: -1.0, z: -5.0 },
            Vec3 { x: 1.0, y: -1.0, z: -5.0 },
            Vec3 { x: 1.0, y: 1.0, z: -5.0 },
            Rgba([0, 0, 255, 255]),
        ),
        Triangle::new(
            Vec3 { x: -1.0, y: -1.0, z: -5.0 },
            Vec3 { x: 1.0, y: 1.0, z: -5.0 },
            Vec3 { x: -1.0, y: 1.0, z: -5.0 },
            Rgba([0, 0, 255, 255]),
        ),
        // Front face (red)
        Triangle::new(
            Vec3 { x: -1.0, y: -1.0, z: -5.0 },
            Vec3 { x: 1.0, y: -1.0, z: -5.0 },
            Vec3 { x: 0.0, y: 0.0, z: -7.0 },
            Rgba([255, 0, 0, 255]),
        ),
        // Right face (green)
        Triangle::new(
            Vec3 { x: 1.0, y: -1.0, z: -5.0 },
            Vec3 { x: 1.0, y: 1.0, z: -5.0 },
            Vec3 { x: 0.0, y: 0.0, z: -7.0 },
            Rgba([0, 255, 0, 255]),
        ),
        // Left face (yellow)
        Triangle::new(
            Vec3 { x: -1.0, y: -1.0, z: -5.0 },
            Vec3 { x: -1.0, y: 1.0, z: -5.0 },
            Vec3 { x: 0.0, y: 0.0, z: -7.0 },
            Rgba([255, 255, 0, 255]),
        ),
    ];

    let rotation_angle = FRAC_PI_4 / 7.0; // 11.25 degrees

    for i in (0..360).step_by(10) {
        for triangle in pyramid.iter() {
            let rotated_triangle = rotate_triangle(&triangle, rotation_angle * (i as f32).to_radians());
            rasterize_triangle(&mut img, &rotated_triangle);
        }

        let filename = format!("output_{:03}.png", i);
        img.save(filename).unwrap();
    }
}

fn rotate_triangle(triangle: &Triangle, angle: f32) -> Triangle {
    let mut rotated_vertices = triangle.vertices;

    for vertex in rotated_vertices.iter_mut() {
        let new_x = vertex.x * angle.cos() - vertex.z * angle.sin();
        let new_z = vertex.x * angle.sin() + vertex.z * angle.cos();
        vertex.x = new_x;
        vertex.z = new_z;
    }

    Triangle {
        vertices: rotated_vertices,
        color: triangle.color,
    }
}

fn rasterize_triangle(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, triangle: &Triangle) {
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let p = Vec3 {
                x: x as f32 / WIDTH as f32 * 2.0 - 1.0,
                y: 1.0 - y as f32 / HEIGHT as f32 * 2.0,
                z: 0.0,
            };

            let barycentric = barycentric_coords(p, triangle);

            if barycentric.x >= 0.0 && barycentric.y >= 0.0 && barycentric.z >= 0.0 {
                img.put_pixel(x, y, triangle.color);
            }
        }
    }
}

fn barycentric_coords(p: Vec3, triangle: &Triangle) -> Vec3 {
    let v0 = triangle.vertices[0];
    let v1 = triangle.vertices[1];
    let v2 = triangle.vertices[2];

    let det_t = (v1.y - v2.y) * (v0.x - v2.x) + (v2.x - v1.x) * (v0.y - v2.y);
    let alpha = ((v1.y - v2.y) * (p.x - v2.x) + (v2.x - v1.x) * (p.y - v2.y)) / det_t;
    let beta = ((v2.y - v0.y) * (p.x - v2.x) + (v0.x - v2.x) * (p.y - v2.y)) / det_t;
    let gamma = 1.0 - alpha - beta;

    Vec3 {
        x: alpha,
        y: beta,
        z: gamma,
    }
}
