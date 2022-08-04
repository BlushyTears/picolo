//! Plot.rs brings plotting functionality aimed to help visualize data
//! 
//! Picolo works with the design philosophy of composability, meaning you can execute using multiple components 
//! to form a layered plot, with a better data representation. (Warning: This is still in the works!)
//! 
//! In this section we have plotting functionality:
//! ```
//!    // The simplest way to plot your data onto a canvas
//!    let a_setting = PlotSettings::default();
//!
//!    let x = vec![0, 500, 200, 300];
//!    let y = vec![0, 100, 200, 300];
//!    plot_tbl(&x, &y, &a_setting, "plot.png");
//!
//!    // Changing a particular setting (source) whilst keeping the default color
//!    let mut b_setting = PlotSettings::default();
//!    b_setting.source = String::from("plot2.png");
//!
//!    let x = vec![0, 500, 200, 300];
//!    let y = vec![0, 100, 200, 300];
//!    plot_tbl(&x, &y, &b_setting, "plot2.png");
//!
//!    // Fully customized setting:
//!    // Color range has to be 0-255
//!    let c_setting = PlotSettings::custom_plot(255, 0, 0, String::from("plog.png"));
//!
//!    let x = vec![0, 100, 200, 300];
//!    let y = vec![0, 100, 200, 300];
//!    plot_tbl(&x, &y, &c_setting, "plot3.png");
//!  
//! ```

use image::{ImageBuffer, Rgb};
use vector2d::Vector2D;

use crate::listops::*;

/// Radius of each square (Future plan to have these in an JSON file)
const SQUARE_RADIUS: i32 = 13;

// s_bound = starting bound (left for x and up for y)
// e_bound = ending bound (right for x and down for y) 
#[derive(Debug)]
struct Bounds {
    smallest: i32,
    largest: i32,
}

#[derive(Debug, Clone)]
pub struct PlotSettings {
    pub plot_color: Color,
}

impl Default for PlotSettings {
    fn default() -> PlotSettings {
        PlotSettings {plot_color: Color::set_vals(50, 50, 230)}
    }
}

// More options to be added later
impl Custom for PlotSettings {
    fn custom_plot(r: u8, g: u8, b: u8) -> PlotSettings {
        PlotSettings {plot_color: Color::set_vals(r, g, b)}
    }
}

pub trait Custom {
    fn custom_plot(r: u8, g: u8, b: u8) -> PlotSettings;
}

/// Offsetted values from the edge of the image in order to help find origin
#[derive(Debug)]
struct OffsetValues {
    origin_x: i32,
    origin_y: i32,
}

trait GetOffsetValues {
    fn get_offset(pair_x: &Bounds, pair_y: &Bounds) -> OffsetValues;
}

impl GetOffsetValues for OffsetValues {
    fn get_offset(pair_x: &Bounds, pair_y: &Bounds) -> OffsetValues {
        let mut origin_x_temp = 0;
        let mut origin_y_temp = pair_y.largest / 2;
        let origin_x_abs = pair_x.largest.abs() + pair_x.smallest.abs();
        let origin_y_abs = pair_y.largest.abs() + pair_y.smallest.abs();

        if pair_x.smallest.is_negative() {
            if pair_x.largest.is_positive() {
                origin_x_temp = origin_x_abs / 2;
            }
            else if pair_x.largest.is_negative() {
                origin_x_temp = (origin_x_abs as f32 * 1.8) as i32; 
            }
        }
        else if pair_x.smallest.is_positive() {
            origin_x_temp = origin_x_abs / 4;
        }

        if pair_y.smallest.is_negative() {
            if pair_y.largest.is_positive() {
                origin_y_temp = origin_y_abs / 2;
            }
            else if pair_y.largest.is_negative() {
                origin_y_temp = (origin_y_abs as f32 * 1.8) as i32; 
            }
        }
        else if pair_y.smallest.is_positive() {
            origin_y_temp = (origin_y_abs as f32 / 1.5) as i32;
        }

        OffsetValues { origin_x: (origin_x_temp), origin_y: (origin_y_temp) }
    }
}

/// Rgb struct
#[derive(Debug, Clone)]
pub struct Color {
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
fn create_canvas(bounds_x: &Bounds, bounds_y: &Bounds, ofv: &OffsetValues) -> ImageBuffer<Rgb<u8>, Vec<u8>> {

    let plot = Color::default();
    let black_clr = Color::set_vals(10, 10, 10);
    let total_width = bounds_x.largest.abs() + bounds_x.smallest.abs();
    let total_height = bounds_y.largest.abs() + bounds_y.smallest.abs();


    let mut imgbuf = image::ImageBuffer::new(total_width as u32, total_height as u32);
    let mut curr_clr = &plot; 

    // Canvas
    for x in 0..total_width {
        for y in 0..total_height {
            let pixel = imgbuf.get_pixel_mut(x.try_into().unwrap(), y.try_into().unwrap());
            *pixel = image::Rgb([plot.red, plot.green, plot.blue]);
        }
    }

    // Lines
    for x in 0..total_width {
        for y in 0..total_height {
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

/// Compresses vector to fit all its datapoints on a graph
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
    float_vec
}

/// Controller function to compress vectors using log
fn vec_controller(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();

    match find_largest_abs_elem(vec.clone()) {
        i if i >= 1000 => new_vec = shrink_vec(&vec),
        i if i <= -1000 => new_vec = shrink_vec(&vec),
        i if i < 1000 && i > 0  => new_vec = vec.to_owned(),
        i if i > -1000 && i < 0 => new_vec = vec.to_owned(),
        _ => println!("Invalid element input"),
    }
    // Add 1 to each element because a potential 0 will mess up drawing of lines (Hacky solution)
    new_vec = new_vec.into_iter().map(|x| x + 1).collect();
    new_vec
}

/// Plotting function that takes in two vectors of type <u32> and draws the plot saved in /images 
pub fn plot_tbl(_vec_x: &Vec<i32>, _vec_y: &Vec<i32>, settings: &PlotSettings, source: &str) {
    let vec_x = vec_controller(_vec_x);
    let vec_y = vec_controller(_vec_y);

    if vec_x.len() != vec_y.len() {panic!("Error: Vector X and Y does not have the same number of elements!");}

    let bounds_x = Bounds{smallest: find_smallest_elem(&vec_x), largest: find_largest_elem(&vec_x)};
    let bounds_y = Bounds{smallest: find_smallest_elem(&vec_y), largest: find_largest_elem(&vec_y)};
    let off_vals = OffsetValues::get_offset(&bounds_x, &bounds_y);

    // let mut imgbuf = create_canvas(find_largest_elem(&vec_x), find_largest_elem(&vec_y), &off_vals);
    let mut imgbuf = create_canvas(&bounds_x, &bounds_y, &off_vals);

    for i in 0..vec_x.len() {
        for j in get_elem_pos(&vec_x[i], &vec_y[i], &off_vals.origin_x, &off_vals.origin_y) {
            let pixel = imgbuf.get_pixel_mut(j.x as u32, j.y as u32);
            *pixel = image::Rgb([settings.plot_color.red, settings.plot_color.green, settings.plot_color.blue]);
        }
    }
    imgbuf.save(source).unwrap();
}

/// Helper function for setting hard-coded values so
/// that origin on graph doesn't start in the upper left corner
fn get_elem_pos(x_pos: &i32, y_pos: &i32, x_clamp: &i32, y_clamp: &i32) -> Vec<Vector2D<i32>>{
    let x = x_pos + x_clamp;
    let y = y_clamp - (*y_pos as f32 / 1.5) as i32;
    gen_sq_map(&x, &y)
}

/// Helper function generating a map for square representing individual elements
/// Subtract by 4 for x and y to account for element offset on plot
fn gen_sq_map(x_pos: &i32, y_pos: &i32) -> Vec<Vector2D<i32>> {
    let mut map = Vec::new();

    for i in *x_pos..SQUARE_RADIUS + *x_pos {
        for j in *y_pos..SQUARE_RADIUS + *y_pos {
            let element: Vector2D<i32> = Vector2D { x: i - 4, y: j - 4};
            map.push(element);        
        }
    }
    map
}
