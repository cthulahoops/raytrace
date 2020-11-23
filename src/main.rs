use std::env;

use raytracelib::camera::{Angle, Camera};
use raytracelib::material::{Dielectric, Diffuse, Light, Metal, Scatter, ScatterResult};
use raytracelib::random::{random_vec3, random_vec3_range};
use raytracelib::vec3::{Color, Point3, Ray, UnitVec3, Vec3};
use raytracelib::world::{Sphere, World};

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use std::f64::INFINITY;

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    };
    if x > max {
        return max;
    };

    if x >= min && x <= max {
        return x;
    }
    0.0
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

fn ray_color(rng: &mut SmallRng, ray: &Ray, world: &World, max_depth: i32) -> Color {
    if max_depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some((hit, sphere)) = world.hit(ray, 0.001, INFINITY) {
        match sphere.material.scatter(rng, ray, &hit) {
            ScatterResult::Reflect(color, ray_out) => {
                return color * ray_color(rng, &ray_out, world, max_depth - 1)
            }
            ScatterResult::Absorb => {
                return Color::new(0.0, 0.0, 0.0);
            }
            ScatterResult::Emit(color) => return color,
        }
    }
    // let light_theta = UnitVec3::cos_theta(ray.direction, Vec3::new(-0.3, -1.0, 0.7).unit_vector());
    // if light_theta <= -0.95 {
    //     Color::new(12.0, 12.0, 12.0)
    // } else {
    Color::new(0.1, 0.1, 0.15)
    // }
    // let t = 0.5 * (ray.direction.y + 1.0);
    // return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn random_scene<R: Rng>(rng: &mut R) -> World {
    let mut world = vec![];

    let ground_material = Box::new(Diffuse {
        albedo: Color::new(0.5, 0.5, 0.5),
    });
    world.push(Sphere {
        center: Point3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground_material,
    });

    for a in -5..5 {
        for b in -5..5 {
            let center = Point3::new(
                2.0 * a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                2.0 * b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material: Box<dyn Scatter> = match rng.gen::<f64>() {
                    x if x < 0.8 => {
                        let albedo = random_vec3(rng) * random_vec3(rng);
                        Box::new(Diffuse { albedo: albedo })
                    }
                    x if x < 0.95 => {
                        let albedo = random_vec3_range(rng, 0.5, 1.0);
                        let fuzz: f64 = rng.gen_range(0.0, 0.5);
                        Box::new(Metal { albedo, fuzz })
                    }
                    _ => Box::new(Dielectric {
                        refractive_index: 1.52,
                    }),
                };
                world.push(Sphere {
                    center,
                    radius: 0.2,
                    material,
                })
            }
        }
    }

    world.push(Sphere {
        center: Point3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Dielectric {
            refractive_index: 1.52,
        }),
    });
    world.push(Sphere {
        center: Point3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Diffuse {
            albedo: Color::new(0.4, 0.2, 0.1),
        }),
    });
    world.push(Sphere {
        center: Point3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Metal {
            albedo: Color::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    });

    World::new(world)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let samples_per_pixel: i64 = args[1].parse::<i64>().unwrap();

    // Image:
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: i64 = 400;
    const IMAGE_HEIGHT: i64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i64;

    let mut rng = SmallRng::from_entropy();

    let world = random_scene(&mut rng);
    //
    // Camera:
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        Angle::from_degrees(20.0),
        16.0 / 9.0,
        aperture,
        dist_to_focus,
    );

    let max_depth = 20;

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanline: {} ", j);

        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                let ray = camera.get_ray(&mut rng, u, v);
                pixel_color = pixel_color + ray_color(&mut rng, &ray, &world, max_depth);
            }

            write_color(pixel_color / samples_per_pixel as f64)
        }
    }
}
