use ray_tracing::camera::Camera;
use ray_tracing::color::Color;
use ray_tracing::hittable::sphere::Sphere;
use ray_tracing::hittable::World;
use ray_tracing::material::dilectric::Dilectric;
use ray_tracing::material::lambertian::Lambertian;
use ray_tracing::material::metal::Metal;
use ray_tracing::utils::random;
use ray_tracing::vec3::Vec3;

fn main() {
    let mut world = World::new();

    let ground_color = Color::new(0.5, 0.5, 0.5);
    let ground_material = Lambertian::new(ground_color);
    let ground = Sphere::new(Vec3::from(0.0, -1000.0, 0.0), 1000.0, ground_material);
    world.add(Box::new(ground));

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_material = random::<f64>();
            let center = Vec3::from(
                a as f64 + 0.9 * random::<f64>(),
                0.2,
                b as f64 + 0.9 * random::<f64>(),
            );

            if choose_material < 0.8 {
                let albedo = Color::random();
                let material = Lambertian::new(albedo);
                let sphere = Sphere::new(center, 0.2, material);
                world.add(Box::new(sphere));
            } else if choose_material < 0.95 {
                let albedo = Color::random();
                let fuzz = random::<f64>();
                let material = Metal::new(albedo, fuzz);
                let sphere = Sphere::new(center, 0.2, material);
                world.add(Box::new(sphere));
            } else {
                let material = Dilectric::new(1.5);
                let sphere = Sphere::new(center, 0.2, material);
                world.add(Box::new(sphere));
            }
        }
    }

    let material1 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    let sphere1 = Sphere::new(Vec3::from(-4.0, 1.0, 0.0), 1.0, material1);
    world.add(Box::new(sphere1));

    let cam = Camera::new();
    cam.render(&world);
}
