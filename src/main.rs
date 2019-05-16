use rustracing::ray::Ray;
use rustracing::vec3::Vec3;

fn color(ray: &Ray) -> Vec3 {
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * &Vec3(1.0, 1.0, 1.0) + t * &Vec3(0.5, 0.7, 1.0);
}

fn main() {
    let nx = 1000;
    let ny = 500;
    println!("P3\n{} {}\n255", nx, ny);

    let lower_left_corner = &Vec3(-2.0, -1.0, -1.0);
    let horizontal = &Vec3(4.0, 0.0, 0.0);
    let vertical = &Vec3(0.0, 2.0, 0.0);
    let origin = &Vec3(0.0, 0.0, 0.0);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;

            let pixel = color(&Ray {
                a: &origin,
                b: &(u * horizontal + v * vertical + lower_left_corner),
            });

            let ir = (pixel.r() * 255.99) as isize;
            let ig = (pixel.g() * 255.99) as isize;
            let ib = (pixel.b() * 255.99) as isize;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
