extern crate rand;

use std::fs::File;
use std::io::Write;

use rand::Rng;

use rustracing::camera::Camera;
use rustracing::hitable::{HitRecord, Hitable};
use rustracing::hitable_list::HitableList;
use rustracing::ray::Ray;
use rustracing::sphere::Sphere;
use rustracing::vec3::Vec3;

fn color(ray: &Ray, world: &Hitable, depth: usize) -> Vec3 {
    let mut rec = HitRecord::new();
    if world.hit(ray, 0.0, std::f32::MAX, &mut rec) {
        let target = rec.p.clone() + rec.normal.clone() + Sphere::rand_point_in_unit_sphere();
        0.5 * &color(
            &Ray {
                a: rec.p.clone(),
                b: target - rec.p.clone(),
            },
            world,
            depth + 1,
        )
    } else {
        let unit_direction = ray.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * &Vec3(1.0, 1.0, 1.0) + t * &Vec3(0.5, 0.7, 1.0)
    }
}

fn main() -> std::io::Result<()> {
    let mut buffer = File::create("output/image.ppm")?;

    let nx = 700;
    let ny = 350;
    let ns = 100;

    buffer.write_fmt(format_args!("P3\n{} {}\n255\n", nx, ny))?;

    let mut world = HitableList::new();
    world
        .list
        .push(Box::new(Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5)));
    world
        .list
        .push(Box::new(Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::new();
    let mut rng = rand::thread_rng();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3(0.0, 0.0, 0.0);
            for _s in 0..ns {
                let u = (rng.gen::<f32>() + i as f32) / nx as f32;
                let v = (rng.gen::<f32>() + j as f32) / ny as f32;

                let ray = camera.get_ray(u, v);
                col += color(&ray, &world, 0);
            }
            col /= ns as f32;

            let ir = (col.r() * 255.99) as isize;
            let ig = (col.g() * 255.99) as isize;
            let ib = (col.b() * 255.99) as isize;
            buffer.write_fmt(format_args!("{} {} {}\n", ir, ig, ib))?;
        }
    }
    Ok(())
}
