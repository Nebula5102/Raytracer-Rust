use std::fs::File;
use std::io::prelude::*;
mod utils;

fn main() -> std::io::Result<()>{

    // Image

    let image_width = 256;
    let image_height = 256;

    let mut f = File::create("img.ppm")?;

    // Render

    write!(f,"P3\n{} {}\n255\n",image_width, image_height)?;

    for j in 0..image_height {
        print!("\rScanlines remaining: {}", image_height-j);
        for i in 0..image_width {
            let pixel_colour = utils::Colour::Vec3(i as f64/(image_width-1) as f64, j as f64/(image_height-1) as f64,0.0);
            let pixel_string = utils::colour_string(pixel_colour);
            write!(f,"{}",pixel_string)?;
        }
    }
    print!("\rDone.                     \n");

    Ok(())
}

