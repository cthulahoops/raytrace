use std::env;

use raytracelib::camera::{Angle, Camera};
use raytracelib::material::{Dielectric, Diffuse, Light, Metal, Material, Scatter, ScatterResult};
use raytracelib::random::{random_vec3, random_vec3_range};
use raytracelib::vec3::{Color, Point3, Ray, Vec3};
use raytracelib::world::{Sphere, World};

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use std::f64::INFINITY;

use rayon::prelude::*;

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

fn simple_scene() -> World {
    let glass = Material::Dielectric(Dielectric {
        refractive_index: 1.52,
    });

    let purple_metal = Material::Metal(Metal {
        albedo: Color::new(0.5, 0.1, 0.5),
        fuzz: 0.0,
    });

    let pink_stone = Material::Diffuse(Diffuse {
        albedo: Color::new(0.8, 0.2, 0.2),
    });

    let light_source = Material::Light(Light {
        color: Color::new(40.0, 40.0, 40.0),
    });

    // World:
    World::new(vec![
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
            center: Point3::new(1.0, -0.25, -0.5),
            radius: 0.25,
            material: glass.clone(),
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
            material: light_source,
        },
        // Sphere {
        //     center: Point3::new(0.7, 0.0, -1.5),
        //     radius: 0.5,
        // },

        // Ground
        Sphere {
            center: Point3::new(0.0, -10000.5, -1.0),
            radius: 10000.0,
            material: Material::Diffuse(Diffuse {
                albedo: Color::new(0.8, 0.8, 0.8),
            }),
        },
    ])
}

fn _random_scene<R: Rng>(rng: &mut R) -> World {
    let mut world = vec![];

    let ground_material = Material::Diffuse(Diffuse {
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
                let material = match rng.gen::<f64>() {
                    x if x < 0.8 => {
                        let albedo = random_vec3(rng) * random_vec3(rng);
                        Material::Diffuse(Diffuse { albedo: albedo })
                    }
                    x if x < 0.95 => {
                        let albedo = random_vec3_range(rng, 0.5, 1.0);
                        let fuzz: f64 = rng.gen_range(0.0, 0.5);
                        Material::Metal(Metal { albedo, fuzz })
                    }
                    _ => Material::Dielectric(Dielectric {
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
        material: Material::Dielectric(Dielectric {
            refractive_index: 1.52,
        }),
    });
    world.push(Sphere {
        center: Point3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Diffuse(Diffuse {
            albedo: Color::new(0.4, 0.2, 0.1),
        }),
    });
    world.push(Sphere {
        center: Point3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Metal(Metal {
            albedo: Color::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    });

    World::new(world)
}

// Image:
const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: i64 = 400;
const IMAGE_HEIGHT: i64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i64;

fn main() {
    let args: Vec<String> = env::args().collect();
    let samples_per_pixel: i64 = args[1].parse::<i64>().unwrap();

    let world = simple_scene();
    //
    // Camera:
    let look_from = Point3::new(2.5, 2.5, 2.5);
    let look_at = Point3::new(1.0, -0.25, -0.5);
    let dist_to_focus = (look_from - look_at).length();
    let aperture = 0.2;

    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        Angle::from_degrees(15.0),
        16.0 / 9.0,
        aperture,
        dist_to_focus,
    );

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    
    let lines : Vec<i64> = (0..IMAGE_HEIGHT).rev().collect();
    let lines : Vec<Vec<Color>> = lines.par_iter().map(|j| {
        eprint!("\rRender scanline: {} ", j);

        render_line(*j, samples_per_pixel, &world, &camera)
    }).collect();

    for line in lines{
        for pixel in line {
            write_color(pixel);
        }
    }
}

fn render_line(j: i64, samples_per_pixel: i64, world: &World, camera: &Camera) -> Vec<Color> {
    let mut rng = SmallRng::from_entropy();
    let max_depth = 20;

    let mut result = vec![];
    for i in 0..IMAGE_WIDTH {
        let mut pixel_color = Color::new(0.0, 0.0, 0.0);

        for _ in 0..samples_per_pixel {
            let u = (i as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
            let v = (j as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
            let ray = camera.get_ray(&mut rng, u, v);
            pixel_color = pixel_color + ray_color(&mut rng, &ray, world, max_depth);
        }

        result.push(pixel_color / samples_per_pixel as f64)
    }
    result
}
