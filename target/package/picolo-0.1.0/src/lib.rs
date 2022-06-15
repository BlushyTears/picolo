//! Picolo is a simple libary for generating all the data prescribed in an image: The dimensions of the image and all pixels
//!
//! [`with_capacity`]: #method.with_capacity
//!
//! # Example
//!
//! Basic usage for reading and translating a complete
//! Image and translating it to a comprehensive table:  
//! ```
//! let pixl_struct = picolo::load_picture("images/ig-icon.png");
//!
//! for i in pixl_struct {
//!     println!("{:?}", i); 
//! }
//! ```

use image::GenericImageView;

/// Rgba struct
#[derive(Debug)]
pub struct Color {
    red: u8,
    blue: u8,
    green: u8,
    alpha: u8,
}

/// Pixel struct that also derives Color struct
#[derive(Debug)]
pub struct Pixel {
    x: u32,
    y: u32,
    color: Color,
}

/// Returns the width of image as u32
pub fn get_width(img: &str) -> u32{
    let img = image::open(img).unwrap();
    img.width()
}

/// Returns height of the image as u32
pub fn get_height(img: &str) -> u32{
    let img = image::open(img).unwrap();
    img.width()
}

/// Takes a borrowed string slice `Str` as its path.
///
/// [`with_capacity`]: #method.with_capacity
///
/// # Example
///
/// A more practical example generating a full table 
/// consisting of each and every pixel
/// from a map called images next to src:
/// ```
/// let foo = "images/ig-icon.png"; 
/// let bar_str = &foo;
/// let pixl_struct = picolo::load_picture(bar_str);
///
/// for i in pixl_struct {
///     println!("x {} y {} red {} green {} blue: {}", i[0].x, i[0].y, 
///     i[0].color.red, i[0].color.green, i[0].color.blue);
/// }
/// ```
pub fn load_picture(img: &str) -> std::vec::Vec<[Pixel; 1]> {

    let mut vec_struct = Vec::new();
    let img = image::open(img).unwrap();

    for i in 0..img.height() {
        for j in 0..img.width() {
            let img_pixel = img.get_pixel(j, i);
            let pxl = [
                Pixel {
                    x: i,
                    y: j,
                    color: Color {
                                red: img_pixel[0], 
                                green: img_pixel[1], 
                                blue: img_pixel[2], 
                                alpha: img_pixel[3]
                            },
                },
            ];
            vec_struct.push(pxl);
        }
    }
    vec_struct
}

