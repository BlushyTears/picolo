//! Picolo is a plotting libary aimed to help carve, read and write data with pixels
//! 
//! Picolo works with the design p&hilosophy of composability, meaning you can execute using multiple components 
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

#[derive(Debug)]
struct OffsetValues {
    origin_x: i32,
    origin_y: i32,
}

trait GetOffsetValues {
    fn get_offset(x: i32, y: i32, has_neg: bool) -> OffsetValues;
}

impl GetOffsetValues for OffsetValues {
    fn get_offset(x: i32, y: i32, has_neg: bool) -> OffsetValues {
        let mut temp_x = 0;
        let mut temp_y = 0;

        if !has_neg {
            temp_x = (x / 4);
            temp_y = (y as f32 / 1.5) as i32;    
        }
        else {
            temp_x = (x / 2);
            temp_y = (y / 2);     
        }
        OffsetValues { origin_x: (temp_x), origin_y: (temp_y) }
    }
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
fn create_canvas(img_x: i32, img_y: i32, ofv: &OffsetValues) -> ImageBuffer<Rgb<u8>, Vec<u8>> {

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
            if x > ofv.origin_x && x < ofv.origin_x + 5 {
                curr_clr = &black_clr;
            }
            else if y > ofv.origin_y && y < ofv.origin_y + 5 {
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
        if i.is_negative() {
            let temp_num = round((switch_sign(&(*i)) as f32).log(1.05), 0) as i32;
            float_vec.push(switch_sign(&temp_num));
        }
        else {
            float_vec.push(round(((*i + temp) as f32).log(1.05), 0) as i32);
        }
    }

    println!("{:?}", float_vec);

    float_vec
}

// Controller function to compress vectors using logs if above 700 or below -200
fn vec_controller(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();

    match find_largest_elem(&vec) {
        i if i >= 1000 => new_vec = shrink_vec(&vec),
        i if i <= -400 => new_vec = shrink_vec(&vec),
        i if i < 1000 => new_vec = vec.to_owned(),
        _ => println!("Invalid element input"),
    }
    new_vec
}

/// Plotting function that takes in two vectors of type <u32> and draws the plot saved in /images 
pub fn plot_tbl(_vec_x: &Vec<i32>, _vec_y: &Vec<i32>) {
    let vec_x = vec_controller(_vec_x);
    let vec_y = vec_controller(_vec_y);
    let mut has_neg = false;

    if vec_x.len() != vec_x.len() {panic!("Error: Vector X and Y does not have the same number of elements!");}


    if has_negative_elem(&_vec_x) || has_negative_elem(&vec_y) {has_neg = true;}
    let off_vals = OffsetValues::get_offset(find_largest_elem(&vec_x), find_largest_elem(&vec_y), has_neg);

    let mut imgbuf = create_canvas(find_largest_elem(&vec_x), find_largest_elem(&vec_y), &off_vals);
    let b_clr = Color::set_vals(50, 50, 230);

    for i in 0..vec_x.len() {
        for j in get_elem_pos(&vec_x[i], &vec_y[i], &off_vals.origin_x, &off_vals.origin_y) {
            let pixel = imgbuf.get_pixel_mut(j.x as u32, j.y as u32);
            *pixel = image::Rgb([b_clr.red, b_clr.green, b_clr.blue]);
        }
    }
    imgbuf.save("images/plot.png").unwrap();
}

// Helper function for setting hard-coded values so
// that origin on graph doesn't start in the upper left corner
fn get_elem_pos(x_pos: &i32, y_pos: &i32, x_clamp: &i32, y_clamp: &i32) -> Vec<Vector2D<i32>>{
    let x = x_pos + x_clamp;
    let y = y_clamp - (*y_pos as f32 / 1.5) as i32;
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