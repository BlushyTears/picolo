use std::vec;

use image::{ImageBuffer, Rgb};
use vector2d::Vector2D;

const CIRCLE_RADIUS: u32 = 13;

/// Rgba struct
#[derive(Debug)]
pub struct Color {
    pub red: u8,
    pub blue: u8,
    pub green: u8,
    pub alpha: u8,
}

trait ColorSet {
    fn set_vals(r: u8, g: u8, b: u8, a: u8) -> Color;
}

impl ColorSet for Color {
    fn set_vals(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { 
            red: (r), 
            green: (g), 
            blue: (b), 
            alpha: (a), 
        }
    }
}

// Hard-coded bg color for now, enum later
impl Default for Color {
    fn default() -> Color {
        Color {
            red: 255,
            blue: 255,
            green: 255,
            alpha: 100,
        }
    }
}

// vec_x: Vec<i32>, vec_y: Vec<i32>
pub fn create_plot(img_x: u32, img_y: u32, y_clamp: &u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {

    let plot = Color::default();
    let black_clr = Color::set_vals(10, 10, 10, 100);
    let mut imgbuf = image::ImageBuffer::new(img_x, img_y);
    let mut curr_clr = &plot; 

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

fn find_largest_elem(vec: &Vec<u32>) -> u32 {
    let min_value = *vec.iter().max().unwrap();
    min_value + 300
}

pub fn draw_data_points(vec_x: &Vec<u32>, vec_y: &Vec<u32>) {

    if vec_x.len() != vec_y.len() {panic!("Error: Length of Vector X and Y are not the same!");}

    let y_clamp = (find_largest_elem(vec_y) as f32 / 1.5) as u32;
    let mut imgbuf = create_plot(find_largest_elem(&vec_x), find_largest_elem(&vec_y), &y_clamp);

    let b_clr = Color::set_vals(50, 50, 230, 100);

    for i in 0..vec_x.len() {
        for j in gen_circ(vec_x[i] as u32, vec_y[i] as u32) {
            let pixel = imgbuf.get_pixel_mut(j.x, j.y);
            *pixel = image::Rgb([b_clr.red, b_clr.green, b_clr.blue]);
        }
    }
    imgbuf.save("plot.png").unwrap();
}

fn gen_circ(x_pos: u32, y_pos: u32) -> Vec<Vector2D<u32>>{
    let x = x_pos + 142;
    let y = 592 - y_pos;
    find_map(&x, &y)
}

fn find_map(x_pos: &u32, y_pos: &u32) -> Vec<Vector2D<u32>> {
    let mut map = Vec::new();

    for i in *x_pos..CIRCLE_RADIUS + *x_pos {
        for j in *y_pos..CIRCLE_RADIUS + *y_pos {
            let element: Vector2D<u32> = Vector2D { x: i, y: j};
            map.push(element);        
        }
    }
    map
}

