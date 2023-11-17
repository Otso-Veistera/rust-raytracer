use std::ops::{Add, Sub, Mul};
use image::{ImageBuffer, Rgb};

#[derive(Clone, Copy)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    fn unit_vector(v: Vec3) -> Vec3 {
        v / v.length()
    }

    fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    fn dot(&self, rhs: Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}



impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3 {
            x: self * vec.x,
            y: self * vec.y,
            z: self * vec.z,
        }
    }
}
use std::ops::Div;
impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        (1.0 / t) * self
    }
}

#[derive(Clone, Copy)]
struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}

#[derive(Clone, Copy)]
struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
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

fn ray_color(ray: &Ray, world: &[Sphere]) -> Vec3 {
    if let Some((t, sphere)) = hit_sphere(world, ray) {
        let hit_point = ray.at(t);
        let normal = Vec3::unit_vector(hit_point - sphere.center);
        return 0.5 * (normal + Vec3::new(1.0, 1.0, 1.0));
    }

    // Background color
    Vec3::new(0.0, 0.0, 0.0)
}

fn main() {
    // Example usage
    let width = 200;
    let height = 100;

    println!("Lets output an image of width {} pixels and height {} pixels", width, height);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let world = vec![
        Sphere {
            center: Vec3::new(0.0, 0.0, -1.0),
            radius: 0.5,
        },
        Sphere {
            center: Vec3::new(0.0, -100.5, -1.0),
            radius: 100.0,
        }];


  //  for j in (0..height).rev() {
      //  for i in 0..width {
          //  let u = i as f64 / (width - 1) as f64;
          //  let v = j as f64 / (height - 1) as f64;

            //let ray = Ray::new(
             //   origin,
             //   lower_left_corner + u * horizontal + v * vertical - origin,
         //   );

            //if let Some((t, sphere)) = hit_sphere(&world, &ray) {
                //let hit_point = ray.at(t);
                //let normal = Vec3::unit_vector(hit_point - sphere.center);
                //let color = 0.5 * (normal + Vec3::new(1.0, 1.0, 1.0));


                //println!("{} {} {}", ir, ig, ib);
           // } //else {
                // Background color
               // println!("0 0 0");
           // }
     //   }
 //   }
    let image_width = 200;
    let image_height = 100;

    let mut img = ImageBuffer::new(image_width, image_height);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);

            let color = ray_color(&ray, &world);

            let ir = (255.999 * color.x) as u8;
            let ig = (255.999 * color.y) as u8;
            let ib = (255.999 * color.z) as u8;

            img.put_pixel(i, j, Rgb([ir, ig, ib]));
        }
    }

    img.save("araytrace.png").expect("Failed to save image");




    }

