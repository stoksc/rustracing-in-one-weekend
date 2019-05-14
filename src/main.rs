fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let r = (i as f32) / (nx as f32);
            let g = (j as f32) / (ny as f32);
            let b = 0.2;
            let ir = (r * 255.99) as isize;
            let ig = (g * 255.99) as isize;
            let ib = (b * 255.99) as isize;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
