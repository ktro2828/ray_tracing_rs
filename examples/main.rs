use std::sync::Arc;

use raytrs::camera::Camera;
use raytrs::color::Color;
use raytrs::geometry::Vec3;
use raytrs::material::ColorTexture;
use raytrs::material::Dilectric;
use raytrs::material::Lambertian;
use raytrs::material::Metal;
use raytrs::render::RenderMode;
use raytrs::render::Renderer;
use raytrs::render::Scene;
use raytrs::shape::Sphere;
use raytrs::utils::random;

fn main() {
    let mut scene = Scene::new(Camera::new(
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        Vec3::new(-2.0, -1.0, -1.0),
        400,
        200,
    ));

    let ground_color = Color::new(0.5, 0.5, 0.5);
    let ground = Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(Box::new(ColorTexture::new(ground_color)))),
    );
    scene.push(Box::new(ground));

    for a in -11..11 {
        for b in -11..11 {
            let material_choice = random::<f64>();
            let center = Vec3::new(
                a as f64 + 0.9 * random::<f64>(),
                0.2,
                b as f64 + 0.9 * random::<f64>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                if material_choice < 0.8 {
                    let albedo = Box::new(ColorTexture::new(Color::random()));
                    let sphere = Sphere::new(center, 0.2, Arc::new(Lambertian::new(albedo)));
                    scene.push(Box::new(sphere));
                } else if material_choice < 0.95 {
                    let albedo = Box::new(ColorTexture::new(Color::random()));
                    let fuzz = random::<f64>();
                    let sphere = Sphere::new(center, 0.2, Arc::new(Metal::new(albedo, fuzz)));
                    scene.push(Box::new(sphere));
                } else {
                    let sphere = Sphere::new(center, 0.2, Arc::new(Dilectric::new(1.5)));
                    scene.push(Box::new(sphere));
                }
            }
        }
    }

    scene.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dilectric::new(1.5)),
    )));
    scene.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(Box::new(ColorTexture::new(Color::new(
            0.4, 0.2, 0.1,
        ))))),
    )));
    scene.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(
            Box::new(ColorTexture::new(Color::new(0.7, 0.6, 0.5))),
            0.0,
        )),
    )));

    let img_basic = scene.render(RenderMode::BASIC);
    img_basic.save("basic.png").unwrap();

    let img_aa = scene.render(RenderMode::AA);
    img_aa.save("aa.png").unwrap();
}
