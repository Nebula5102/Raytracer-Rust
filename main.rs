use std::fs::File;
use std::io::prelude::*;
mod utils;

fn hit_sphere(center: utils::Point3, radius: f64, r: &utils::Ray) -> bool{
    let oc = r.origin() - center;
    let a = utils::dot(r.direction(),r.direction());
    let b = 2.0 * utils::dot(oc,r.direction());
    let c = utils::dot(oc,oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    return discriminant >= 0.0;
}

fn ray_colour(r: utils::Ray) -> utils::Colour {
    if hit_sphere(utils::Point3::Vec3(0.0,0.0,-1.0),0.5,&r) {
        return utils::Colour::Vec3(1.0,0.0,0.0);
    }
    let unit_direction = r.direction().unit_vector();
    let a = 0.5*(unit_direction.y() + 1.0);
    return (1.0-a)*utils::Colour::Vec3(1.0,1.0,1.0) + a*utils::Colour::Vec3(0.5,0.7,1.0);
}

fn main() -> std::io::Result<()>{

    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    
    let mut image_height = f64::from(image_width) / aspect_ratio;
    image_height = f64::max(image_height, 1.0);

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (f64::from(image_width)/image_height);
    let camera_center = utils::Point3::Vec3(0.0,0.0,0.0);

    let viewport_u = utils::Vec3::Vec3(viewport_width,0.0,0.0);
    let viewport_v = utils::Vec3::Vec3(0.0,-viewport_height,0.0);

    let pixel_delta_u = viewport_u / f64::from(image_width);
    let pixel_delta_v = viewport_v / f64::from(image_height);

    let viewport_upper_left = camera_center - utils::Vec3::Vec3(0.0,0.0,focal_length) - viewport_u/2.0 - viewport_v/2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut f = File::create("img.ppm")?;

    // Render

    write!(f,"P3\n{} {}\n255\n",image_width, image_height)?;

    for j in 0..image_height as i64 {
        print!("\rScanlines remaining: {}", image_height - j as f64);
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (f64::from(i) * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = utils::Ray::Ray(&camera_center,&ray_direction);
            let pixel_colour = ray_colour(ray);
            let pixel_string = utils::colour_string(pixel_colour);
            write!(f,"{}",pixel_string)?;
        }
    }
    print!("\rDone.                     \n");

    Ok(())
}

