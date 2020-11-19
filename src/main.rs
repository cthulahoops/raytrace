use std::env;

use raytracelib::camera::Camera;
use raytracelib::material::{Dielectric, Diffuse, Hit, Metal, Sphere};
use raytracelib::vec3::{Color, Point3, Ray, Vec3};

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

// use std::f64::consts::PI;
use std::f64::INFINITY;

// fn degrees_to_radians(degrees: f64) -> f64 {
//     return degrees * PI / 180.0;
// }

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
        to_8bit_color(color.x.sqrt()),
        to_8bit_color(color.y.sqrt()),
        to_8bit_color(color.z.sqrt())
    )
}

type World = Vec<Sphere>;

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

fn hit_world<'a>(world: &'a World, ray: &Ray, t_min: f64, t_max: f64) -> Option<(Hit, &'a Sphere)> {
    let mut closest_so_far = t_max;
    let mut best_so_far: Option<(Hit, &Sphere)> = None;
    for sphere in world {
        if let Some(hit) = hit_sphere(&sphere, ray, t_min, closest_so_far) {
            closest_so_far = hit.t;
            best_so_far = Some((hit, sphere));
        }
    }
    best_so_far
}

fn ray_color(rng: &mut SmallRng, ray: &Ray, world: &World, max_depth: i32) -> Color {
    if max_depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some((hit, sphere)) = hit_world(world, ray, 0.001, INFINITY) {
        match sphere.material.scatter(rng, ray, &hit) {
            Some((color, ray_out)) => {
                return color * ray_color(rng, &ray_out, world, max_depth - 1)
            }
            None => {
                return Color::new(0.0, 0.0, 0.0);
            }
        }
    }
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let samples_per_pixel: i64 = args[1].parse::<i64>().unwrap();

    // Image:
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i64 = 400;
    const IMAGE_HEIGHT: i64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i64;

    let glass = Box::new(Dielectric {
        refractive_index: 1.52,
    });

    let purple_metal = Box::new(Metal {
                albedo: Color::new(0.5, 0.1, 0.5),
                fuzz: 0.0,
            });

    let pink_stone = Box::new(Diffuse {
                albedo: Color::new(0.8, 0.2, 0.2),
            });

    // World:
    let world = vec![
        // Sphere {
        //     center: Point3::new(-0.7, 0.0, -1.5),
        //     radius: 0.5,
        // },
        Sphere {
            center: Point3::new(0.5, 0.0, -1.0),
            radius: 0.5,
            material: purple_metal,
        },
        Sphere {
            center: Point3::new(-0.5, 0.0, -1.0),
            radius: 0.5,
            material: pink_stone.clone(),
        },
        Sphere {
            center: Point3::new(0.0, -0.25, -0.5),
            radius: 0.25,
            material: glass,
        },
        Sphere {
            center: Point3::new(0.0, -0.25, -0.5),
            radius: 0.2,
            material: pink_stone,
        },
        Sphere {
            center: Point3::new(0.0, 0.0, 2.0),
            radius: 0.5,
            material: Box::new(Diffuse {
                albedo: Color::new(0.2, 0.8, 0.2),
            }),
        },
        // Sphere {
        //     center: Point3::new(0.7, 0.0, -1.5),
        //     radius: 0.5,
        // },

        // Ground
        Sphere {
            center: Point3::new(0.0, -10000.5, -1.0),
            radius: 10000.0,
            material: Box::new(Metal {
                albedo: Color::new(0.5, 0.5, 0.5),
                fuzz: 0.3,
            }),
        },
    ];

    // Camera:
    let camera = Camera::new();

    let max_depth = 20;

    let mut rng = SmallRng::from_entropy();

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanline: {} ", j);

        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);

            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&mut rng, &ray, &world, max_depth);
            }

            write_color(pixel_color / samples_per_pixel as f64)
        }
    }
}
