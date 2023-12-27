use std::fs::File;
use std::io::Write;
use std::sync::Arc;

use raytrs::color::Color;
use raytrs::geometry::Vec3;
use raytrs::material::Dilectric;
use raytrs::material::Lambertian;
use raytrs::material::Metal;
use raytrs::render::Renderer;
use raytrs::render::Scene;
use raytrs::shape::Sphere;
use raytrs::utils::random;

fn main() {
    let mut scene = Scene::new();

    let ground_color = Color::new(0.5, 0.5, 0.5);
    let ground = Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(ground_color)),
    );
    scene.push(Box::new(ground));

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_material = random::<f64>();
            let center = Vec3::new(
                a as f64 + 0.9 * random::<f64>(),
                0.2,
                b as f64 + 0.9 * random::<f64>(),
            );

            if choose_material < 0.8 {
                let albedo = Color::random();
                let sphere = Sphere::new(center, 0.2, Arc::new(Lambertian::new(albedo)));
                scene.push(Box::new(sphere));
            } else if choose_material < 0.95 {
                let albedo = Color::random();
                let fuzz = random::<f64>();
                let sphere = Sphere::new(center, 0.2, Arc::new(Metal::new(albedo, fuzz)));
                scene.push(Box::new(sphere));
            } else {
                let sphere = Sphere::new(center, 0.2, Arc::new(Dilectric::new(1.5)));
                scene.push(Box::new(sphere));
            }
        }
    }

    let sphere1 = Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))),
    );
    scene.push(Box::new(sphere1));

    let ppm_str = scene.render();

    let mut out_file = File::create("output.ppm").unwrap();
    write!(out_file, "{}", ppm_str).unwrap()
}
