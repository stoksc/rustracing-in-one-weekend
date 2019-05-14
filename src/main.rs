use rustracing::vec3::Vec3;

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let v = Vec3((i as f32) / (nx as f32), (j as f32) / (ny as f32), 0.2);
            let ir = (v[0] * 255.99) as isize;
            let ig = (v[1] * 255.99) as isize;
            let ib = (v[2] * 255.99) as isize;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
