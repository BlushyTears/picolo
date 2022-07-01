//! Picolo is a simple libary for reading all the data prescribed in an image: The dimensions of the image and all pixels
//!
//! [`with_capacity`]: #method.with_capacity
//!
//! # Example
//!
//! Basic usage for reading and translating a complete
//! Image and translating it to a comprehensive table:  
//! ```
//! use picolo::readimg::load_picture;
//! // Function takes in image path and precision variable:
//! // 1 = count all pixels, 2 = count half and so on:
//! let pixl_struct = load_picture("images/ig-icon.png", 2);
//!
//! for i in pixl_struct {
//!     println!("{:?}", i); 
//! }
//! ```

use image::GenericImageView;

/// Rgba struct
#[derive(Debug)]
pub struct Color {
    pub red: u8,
    pub blue: u8,
    pub green: u8,
    pub alpha: u8,
}

/// Pixel struct that also derives Color struct
#[derive(Debug)]
pub struct Pixel {
    pub x: u32,
    pub y: u32,
    pub color: Color,
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

/// Takes a borrowed string slice `Str` as its path and usize for precision.
///
/// [`with_capacity`]: #method.with_capacity
///
/// # Example
///
/// A more practical example generating a full table 
/// consisting of each and every pixel
/// from a map called images next to src:
/// ```
/// use picolo::readimg::load_picture;
/// let foo = "images/icon.png"; 
/// let bar_str = &foo;
/// let pixl_struct = load_picture(bar_str, 2);
///
/// for i in pixl_struct {
///     println!("x {} y {} red {} green {} blue: {}", i[0].x, i[0].y, 
///     i[0].color.red, i[0].color.green, i[0].color.blue);
/// }
/// ```

pub fn load_picture(img: &str, precision: u32) -> std::vec::Vec<[Pixel; 1]> {

    let mut vec_struct = Vec::new();
    let img = image::open(img).unwrap();

    if precision == 0 || precision > img.height() || precision > img.width() {
        panic!("Precision cannot be value {} because it's either 0
        or larger than size of the image height {} or image width {}",
        precision, img.height(), img.width() ); 
    };

    for i in 0..(img.height() / precision) {
        for j in 0..(img.width() / precision) {
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
    };
    vec_struct
}