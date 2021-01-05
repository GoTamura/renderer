use image;
use renderer::geometry::Normal3f;
use renderer::geometry::Point3f;
use renderer::geometry::Ray;
use renderer::geometry::Vector3f;
use renderer::light::*;
use renderer::material::*;
use renderer::point3f;
use renderer::spe;
use renderer::spectrum::*;
use renderer::vec3f;

const WIDTH: u32 = 2000;
const HEIGHT: u32 = 2000;
const EYE: Point3f = point3f!(0., 0., 5.);
const SPHERE_CENTER: Point3f = point3f!(0.);
const SPHERE_RADIUS: f64 = 1.;
const NO_HIT: f64 = std::f64::INFINITY;

const MATERIAL: Material = Material {
    diffuse: spe!(1., 0.5, 0.25),
};

const LIGHT: Light = Light {
    pos: point3f!(10.),
    power: spe!(4000.),
};

const PI: f64 = 3.1415926653589;

fn calc_primary_ray(x: u32, y: u32) -> Vector3f {
    let image_plane = HEIGHT as f64;

    let dx = x as f64 + 0.5 - WIDTH as f64 / 2.;
    let dy = -(y as f64 + 0.5 - HEIGHT as f64 / 2.);
    let dz = -image_plane;

    let ray_direction = vec3f!(dx, dy, dz);
    ray_direction.noramlize()
}

fn calc_pixel_color(x: u32, y: u32) -> Spectrum {
    let ray = Ray {
        o: EYE,
        d: calc_primary_ray(x, y),
    };

    let t = intersect_ray_sphere(&ray, SPHERE_CENTER, SPHERE_RADIUS);
    if t == NO_HIT {
        return spe!(0.0);
    }
    let p = ray.o + ray.d * t;
    let n = (p - SPHERE_CENTER).noramlize();

    diffuse_lighting(p, n, MATERIAL.diffuse, LIGHT)
}

fn intersect_ray_sphere(ray: &Ray, sphere_center: Point3f, sphere_radius: f64) -> f64 {
    let v = ray.o - sphere_center;
    let b = ray.d.dot(v);
    let c = v.dot(v) - sphere_radius * sphere_radius;
    let d = b * b - c;
    if d >= 0. {
        let s = d.sqrt();
        let mut t = -b - s;
        if t <= 0. {
            t = -b + s;
        }
        if 0. < t {
            return t;
        }
    }

    NO_HIT
}

fn diffuse_lighting(p: Point3f, n: Normal3f, diffuse_color: Spectrum, light: Light) -> Spectrum {
    let v = light.pos - p;
    let l = v.noramlize();

    let dot = n.dot(l);

    if dot > 0. {
        let r = v.length();
        let factor = dot / (4. * PI * r * r);
        (light.power * diffuse_color * factor).min(spe!(1.0))
    } else {
        BLACK
    }
}

fn main() {
    let mut imgbuf = image::ImageBuffer::new(WIDTH, HEIGHT);
    let v: Vector3f;

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let c = calc_pixel_color(x, y);
        let r = (c.r * 255.) as u8;
        let g = (c.g * 255.) as u8;
        let b = (c.b * 255.) as u8;
        *pixel = image::Rgb([r, g, b]);
    }

    imgbuf.save("test.png").unwrap();
    println!("Rendering!");
}
