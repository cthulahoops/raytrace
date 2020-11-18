mod vec3;

use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

use std::f64::consts::PI;
use std::f64::INFINITY;
use vec3::{Color, Point3, Ray, Vec3};

fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * PI / 180.0;
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    };
    if x > max {
        return max;
    };
    x
}

fn to_8bit_color(c: f64) -> i32 {
    (256.0 * clamp(c, 0.0, 0.999)) as i32
}

fn write_color(color: Color) {
    println!(
        "{} {} {}",
        to_8bit_color(color.x),
        to_8bit_color(color.y),
        to_8bit_color(color.z)
    )
}

struct Sphere {
    center: Point3,
    radius: f64,
}

type World = Vec<Sphere>;

enum Face {
    Front,
    Back,
}

struct Hit {
    point: Point3,
    normal: Vec3,
    t: f64,
    face: Face,
}

impl Hit {
    fn new(root: f64, ray: &Ray, sphere: &Sphere) -> Self {
        let point = ray.at(root);

        let outward_normal = (point - sphere.center) / sphere.radius;
        let face = if Vec3::dot(ray.direction, outward_normal) < 0.0 {
            Face::Front
        } else {
            Face::Back
        };
        Hit {
            t: root,
            point: point,
            normal: match face {
                Face::Front => outward_normal,
                Face::Back => -1.0 * outward_normal,
            },
            face: face,
        }
    }
}

fn hit_sphere(sphere: &Sphere, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
    let center = sphere.center;
    let radius = sphere.radius;

    let oc = ray.origin - center;
    let a = ray.direction.length_squared();
    let half_b = Vec3::dot(oc, ray.direction);
    let c = oc.length_squared() - radius * radius;

    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return None;
    }
    let sqrtd = discriminant.sqrt();

    let mut root = (-half_b - sqrtd) / a;

    if root < t_min || root > t_max {
        root = (-half_b + sqrtd) / a;
        if root < t_min || root > t_max {
            return None;
        }
    }

    Some(Hit::new(root, ray, sphere))
}

fn hit_world(world: &World, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
    let mut closest_so_far = t_max;
    let mut best_so_far: Option<Hit> = None;
    for sphere in world {
        if let Some(hit) = hit_sphere(&sphere, ray, t_min, closest_so_far) {
            closest_so_far = hit.t;
            best_so_far = Some(hit);
        }
    }
    best_so_far
}

struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.9 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical,
        )
    }
}

fn ray_color(ray: &Ray, world: &World) -> Color {
    if let Some(hit) = hit_world(world, ray, 0.0, INFINITY) {
        return 0.5 * (hit.normal + Color::new(1.0, 1.0, 1.0));
        //        return Color::new(1.0, 0.0, 0.0);
    }
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn main() {
    // Image:
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i64 = 400;
    const IMAGE_HEIGHT: i64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i64;

    // World:
    let world = vec![
        // Sphere {
        //     center: Point3::new(-0.7, 0.0, -1.5),
        //     radius: 0.5,
        // },
        Sphere {
            center: Point3::new(0.0, 0.0, -1.0),
            radius: 0.5,
        },
        // Sphere {
        //     center: Point3::new(0.7, 0.0, -1.5),
        //     radius: 0.5,
        // },

        // Ground
        Sphere {
            center: Point3::new(0.0, -100.5, -1.0),
            radius: 100.0,
        },
    ];

    // Camera:
    let camera = Camera::new();

    let samples_per_pixel = 100;

    let mut rng = SmallRng::from_entropy();

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanline: {}", j);

        for i in 0..IMAGE_WIDTH {
            let mut pixel_color =Vec3::new(0.0, 0.0, 0.0);

            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&ray, &world);
            }

            write_color(pixel_color / samples_per_pixel as f64)
        }
    }
}
