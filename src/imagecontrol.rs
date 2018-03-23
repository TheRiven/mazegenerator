extern crate image;

use std::collections::HashMap;
use std::fs::File;

use maze::Node;

pub fn generate_image(height: u32, width: u32, node_map: HashMap<(u64, u64), Node>) {
    // Create image buffer
    let mut img_buffer = image::ImageBuffer::new(width, height);

    // Iterate over the coords and pixels in the image
    for (x, y, pixel) in img_buffer.enumerate_pixels_mut() {
        let position = (x as u64, y as u64);
        let node = node_map.get(&position).unwrap();

        if node.is_wall() {
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
