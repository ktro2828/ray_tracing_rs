/// Reference: https://raytracing.github.io/books/RayTracingInOneWeekend.html
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use raytrs::color::Color;
use raytrs::geometry::Vec3;
use raytrs::ray::Ray;

fn sample_ppm() {
    let image_width = 256;
    let image_height = 256;

    let mut out_file = File::create("sample1.ppm").unwrap();
    let ppm_header = format!("P3\n{} {}\n255\n", image_width, image_height);
    writeln!(out_file, "{}", ppm_header).unwrap();

    for j in 0..image_height {
        for i in 0..image_width {
            let r = (i / (image_width - 1)) as f32;
            let g = (j / (image_height - 1)) as f32;
            let b = 0.0_f32;

            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;

            let line = format!("{} {} {}\n", ir, ig, ib);
            writeln!(out_file, "{}", line).unwrap();
        }
    }
}

///
/// Sample code for ray
///
/// ```
/// --------------------------> ViewPort u
/// | Q______________________
/// | | . . . . . . . . . . .|
/// | | P00 . . . . . . . . .|_
/// | | . . . . . . . . . . .|_| delta v
/// | | |_|                  |
/// | | delta u              |
/// | |______________________|
/// v
/// ViewPort v
///
/// * Q         ...The viewport upper left corner
/// * P00       ...The pixel (0, 0) location
/// * Vu,Vv     ...The viewport vectors
/// * delta u,v ...The pixel deltas
/// ```
fn sample_ray(path: PathBuf) {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // calculate the image height, and ensure that it's at least 1.
    let image_height = if (image_width as f64 / aspect_ratio) < 0.0 {
        1
    } else {
        (image_width as f64 / aspect_ratio) as i32
    };

    // camera
    let focal_length = 1.0;
    let viewport_height = 1.0;
    let viewport_width = viewport_height * (image_width / image_height) as f64;
    let camera_center = Vec3::new(0.0, 0.0, 0.0);

    // calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width as f64, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height as f64, 0.0);

    // calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - (viewport_u + viewport_v) * 0.5;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    // render
    let mut out_file = File::create(path).unwrap();
    let ppm_header = format!("P3\n{} {}\n255\n", image_width, image_height);
    writeln!(out_file, "{}", ppm_header).unwrap();

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);
            let px_color = get_ray_color(&ray);
            write_color(px_color, &mut out_file);
        }
    }
}

fn get_ray_color(ray: &Ray) -> Color {
    let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let n = (ray.at(t) - Vec3::new(0.0, 0.0, -1.0)).as_unit();
        Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5
    } else {
        let unit_dir = ray.direction().as_unit();
        let a = (unit_dir.y() + 1.0) * 0.5;
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}

fn write_color(color: Color, out_file: &mut File) {
    let ir = (color.r * 255.99) as i32;
    let ig = (color.g * 255.99) as i32;
    let ib = (color.b * 255.99) as i32;
    let ppm_str = format!("{} {} {}\n", ir, ig, ib);
    writeln!(out_file, "{}", ppm_str).unwrap();
}

fn hit_sphere(center: Vec3, radius: f64, ray: &Ray) -> f64 {
    let oc = *ray.origin() - center;
    let a = ray.direction().dot(*ray.direction());
    let b = oc.dot(*ray.direction()) * 2.0;
    let c = oc.dot(oc) - radius.powi(2);
    let discriminant = b.powi(2) - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

/// Ray-Sphere intersection sample.
///
/// The equation for a sphere is
/// x^2 + y^2 + z^2 = r^2
fn sample_sphere() {
    let path = PathBuf::from("sample3.ppm");
    sample_ray(path)
}

fn main() {
    sample_ppm();
    sample_ray(PathBuf::from("sample2.ppm"));
    sample_sphere();
}
