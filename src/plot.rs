//! Picolo is a plotting libary aimed to help carve, read and write data with pixels
//! 
//! Picolo works with the design philosophy of composability, meaning you can execute using multiple components 
//! to form a layered plot, with a better data representation. (Warning: This is still in the works!)
//! 
//! In this section we have plotting functionality:
//! ```
//!     // The simplest way to plot your data onto a canvas
//!     use picolo::plot::plot_tbl;
//!     let x = vec![0, 152, 1000];
//!     let y = vec![0, 152, 490];
//!     plot_tbl(&x, &y);
//!     // Output found in /images/plot.png
//! ```

use image::{ImageBuffer, Rgb};
use vector2d::Vector2D;

/// Radius of each square (Future plan to have these in an JSON file)
const SQUARE_RADIUS: u32 = 13;

/// Rgba struct
#[derive(Debug)]
struct Color {
    pub red: u8,
    pub blue: u8,
    pub green: u8,
}

/// Trait for setting custom color
trait ColorSet {
    fn set_vals(r: u8, g: u8, b: u8) -> Color;
}

/// Implement setting of custom color
impl ColorSet for Color {
    fn set_vals(r: u8, g: u8, b: u8) -> Color {
        Color { 
            red: (r), 
            green: (g), 
            blue: (b), 
        }
    }
}

/// Default color
impl Default for Color {
    fn default() -> Color {
        Color {
            red: 255,
            blue: 255,
            green: 255,
        }
    }
}

// Creates a basic canvas. 
// Returns: Image buffer for other functions to operate on
fn create_plot(img_x: u32, img_y: u32, y_clamp: &u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {

    let plot = Color::default();
    let black_clr = Color::set_vals(10, 10, 10);
    let mut imgbuf = image::ImageBuffer::new(img_x, img_y);
    let mut curr_clr = &plot; 

    // Canvas
    for x in 0..img_x {
        for y in 0..img_y {
            let pixel = imgbuf.get_pixel_mut(x, y);
            *pixel = image::Rgb([plot.red, plot.green, plot.blue]);
        }
    }

    // Lines
    for x in 0..img_x {
        for y in 0..img_y {
            if x > 150 && x < 155 {
                curr_clr = &black_clr;
            }
            else if y > *y_clamp && y < *y_clamp + 5 {
                curr_clr = &black_clr;
            }
            else {curr_clr = &plot;}
            let pixel = imgbuf.get_pixel_mut(x, y);
            *pixel = image::Rgb([curr_clr.red, curr_clr.green, curr_clr.blue]);
        }
    }
    imgbuf
}

// Helper function for finding largest element in a given vector
fn find_largest_elem(vec: &Vec<u32>) -> u32 {
    let min_value = *vec.iter().max().unwrap();
    min_value + 300
}

/// Plotting function that takes in two vectors of type <u32> and draws the plot saved in /images 
pub fn plot_tbl(vec_x: &Vec<u32>, vec_y: &Vec<u32>) {

    if vec_x.len() != vec_y.len() {panic!("Error: Length of Vector X and Y are not the same!");}

    let y_clamp = (find_largest_elem(vec_y) as f32 / 1.5) as u32;
    let mut imgbuf = create_plot(find_largest_elem(&vec_x), find_largest_elem(&vec_y), &y_clamp);

    let b_clr = Color::set_vals(50, 50, 230);

    for i in 0..vec_x.len() {
        for j in get_sq_pos(vec_x[i], vec_y[i], &y_clamp) {
            let pixel = imgbuf.get_pixel_mut(j.x, j.y);
            *pixel = image::Rgb([b_clr.red, b_clr.green, b_clr.blue]);
        }
    }
    imgbuf.save("plot.png").unwrap();
}

// Helper function for plot() fn
fn get_sq_pos(x_pos: u32, y_pos: u32, y_clamp: &u32) -> Vec<Vector2D<u32>>{
    let x = x_pos + 142;
    let y = y_clamp - (y_pos as f32 / 1.5) as u32;
    gen_map(&x, &y)
}

// Helper function generating a map for gen_sq_pos() fn
fn gen_map(x_pos: &u32, y_pos: &u32) -> Vec<Vector2D<u32>> {
    let mut map = Vec::new();

    for i in *x_pos..SQUARE_RADIUS + *x_pos {
        for j in *y_pos..SQUARE_RADIUS + *y_pos {
            let element: Vector2D<u32> = Vector2D { x: i, y: j};
            map.push(element);        
        }
    }
    map
}