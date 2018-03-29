extern crate image;

use std::collections::HashSet; 
use std::fs::File;

pub fn generate_image(height: u32, width: u32, node_map: &HashSet<(u32, u32)>) {
    // Create image buffer
    let mut img_buffer = image::ImageBuffer::new(width, height);

    // Iterate over the coords and pixels in the image
    for (x, y, pixel) in img_buffer.enumerate_pixels_mut() {
        let position = (x as u32, y as u32);
        let node = node_map.get(&position);

        if node.is_none() {
            *pixel = image::Luma([0u8]);
        } else {
            *pixel = image::Luma([255u8])
        }
    }

    // Send the image
    let ref mut fout = File::create("maze.png").unwrap();
    image::ImageLuma8(img_buffer)
        .save(fout, image::PNG)
        .unwrap();
}

pub fn generate_solved_image(height: u32, width: u32, node_map: &HashSet<(u32, u32)>, path: Vec<&(u32, u32)>) {
    // Create image buffer
    let mut img_buffer = image::ImageBuffer::new(width, height);

    // Iterate over the coords and pixels in the image
    for (x, y, pixel) in img_buffer.enumerate_pixels_mut() {
        let position = (x as u32, y as u32);
        //let node = node_map.get(&position);

        if !node_map.contains(&position) {
            *pixel = image::Rgb([0u8, 0, 0]);
        } else {
            *pixel = image::Rgb([255u8, 255, 255])
        }
    }

    // index / length of array, color * result
    for (index, pos) in path.iter().enumerate(){
        let position = (pos.0 as u32, pos.1 as u32);
        let pixel = img_buffer.get_pixel_mut(position.0, position.1);

        let percent = index as f32 / path.len() as f32;
        let color_b = (255f32 * percent) as u8; 
        let color_r = 255 - color_b;

        *pixel = image::Rgb([color_r, 0, color_b]);
    }

    // Send the image
    let ref mut fout = File::create("solved-maze.png").unwrap();
    image::ImageRgb8(img_buffer)
        .save(fout, image::PNG)
        .unwrap();
}
