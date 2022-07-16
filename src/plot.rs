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

use crate::listops::*;

/// Radius of each square (Future plan to have these in an JSON file)
const SQUARE_RADIUS: i32 = 13;

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
fn create_plot(img_x: i32, img_y: i32, y_clamp: &i32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {

    let plot = Color::default();
    let black_clr = Color::set_vals(10, 10, 10);
    let mut imgbuf = image::ImageBuffer::new(img_x as u32, img_y as u32);
    let mut curr_clr = &plot; 

    // Canvas
    for x in 0..img_x {
        for y in 0..img_y {
            let pixel = imgbuf.get_pixel_mut(x.try_into().unwrap(), y.try_into().unwrap());
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
            let pixel = imgbuf.get_pixel_mut(x.try_into().unwrap(), y.try_into().unwrap());
            *pixel = image::Rgb([curr_clr.red, curr_clr.green, curr_clr.blue]);
        }
    }
    imgbuf
}

// Compresses vector to fit all its datapoints on a graph
fn shrink_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut float_vec = Vec::new();
    let mut temp = 0;

    for i in vec {
        if *i == 0 {temp = 1;} else {temp = 0;}
        float_vec.push(round(((*i + temp) as f32).log(1.05), 0) as i32);
    }

    println!("{:?}", float_vec);

    float_vec
}

// Controller function to compress vectors using logs if above 700
fn vec_controller(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();

    match find_largest_elem(&vec) {
        i if i >= 1000 => new_vec = shrink_vec(&vec),
        i if i < 1000 => new_vec = vec.to_owned(),
        _ => println!("lol"),
    }

    new_vec
}

/// Plotting function that takes in two vectors of type <u32> and draws the plot saved in /images 
pub fn plot_tbl(_vec_x: &Vec<i32>, _vec_y: &Vec<i32>) {
    let vec_x = vec_controller(_vec_x);
    let vec_y = vec_controller(_vec_y);


    if vec_x.len() != vec_y.len() {panic!("Error: Length of Vector X and Y are not the same!");}

    let y_clamp = (find_largest_elem(&vec_y) as f32 / 1.5) as i32;
    let mut imgbuf = create_plot(find_largest_elem(&vec_x), find_largest_elem(&vec_y), &y_clamp.try_into().unwrap());
    let b_clr = Color::set_vals(50, 50, 230);

    for i in 0..vec_x.len() {
        for j in get_sq_pos(vec_x[i], vec_y[i], &y_clamp) {
            let pixel = imgbuf.get_pixel_mut(j.x as u32, j.y as u32);
            *pixel = image::Rgb([b_clr.red, b_clr.green, b_clr.blue]);
        }
    }
    imgbuf.save("images/plot.png").unwrap();
}

// Helper function for setting hard-coded values so
// that origin on graph doesn't start in the upper left corner
fn get_sq_pos(x_pos: i32, y_pos: i32, y_clamp: &i32) -> Vec<Vector2D<i32>>{
    let x = x_pos + 142;
    let y = y_clamp - (y_pos as f32 / 1.5) as i32;
    gen_map(&x, &y)
}

// Helper function generating a map for square representing individual elements
fn gen_map(x_pos: &i32, y_pos: &i32) -> Vec<Vector2D<i32>> {
    let mut map = Vec::new();

    for i in *x_pos..SQUARE_RADIUS + *x_pos {
        for j in *y_pos..SQUARE_RADIUS + *y_pos {
            let element: Vector2D<i32> = Vector2D { x: i, y: j};
            map.push(element);        
        }
    }
    map
}