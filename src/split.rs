use image::{ImageBuffer, Rgb, GenericImageView};
use crate::readimg::*;


// Todo:
// Take in 1 picture source, returns X number of split images
// If you have 200x200 and want to split into 4 parts, then it will be four 50x50 images
// If image is 1000x1000 then images will be split into 10 parts then each will be 100x100

pub fn split_img(src: &str, size_x: &u32, size_y: &u32) {
    // let mut vec_struct = Vec::new();

    // get_width("plot.png");

    if get_height(src) as f32 % *size_x as f32 != 0.0 || 
    get_width(src) as f32 % *size_y as f32 != 0.0 {
    panic!("Dimensions of {} Divided by x: {} or y: {} causes decimal
        arithmetic. Please make sure contents of image is a perfect 
        square and its denominators do not generate a fraction",
        src, size_x, size_y);
    } 
    
    let img = load_picture(src, 100);
    let x_iter = get_height(src) / size_x;
    let y_iter = get_width(src) / size_y;

    for i in 0..x_iter {
        for j in (0..i * size_x).step_by(*size_x as usize) {
            println!("{}", j);
        }
    }
}

// Helper function for initiating image based on 
fn draw_image() {
    // let img = image::open(src).unwrap();

}


