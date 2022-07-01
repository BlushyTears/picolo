use image::{ImageBuffer, Rgb};
use vector2d::Vector2D;

const IMAGE_X: u32 = 800;
const IMAGE_Y: u32 = 800;
const CIRCLE_RADIUS: u32 = 13;

/// Rgba struct
#[derive(Debug)]
pub struct Color {
    pub red: u8,
    pub blue: u8,
    pub green: u8,
    pub alpha: u8,
}

#[derive(Debug)]
struct Plot {
    imgx: u32,
    imgy: u32,
    bg_color: Color,
}

// Hard-coded bg color for now, enum later
impl Default for Plot {
    fn default() -> Plot {
        Plot {
            imgx: IMAGE_X,
            imgy: IMAGE_Y,
            bg_color: Color {
                red: 255,
                blue: 255,
                green: 255,
                alpha: 100,
            }
        }
    }
}

// vec_x: Vec<i32>, vec_y: Vec<i32>
pub fn create_plot() -> ImageBuffer<Rgb<u8>, Vec<u8>> {

    let plot = Plot::default();
    let mut imgbuf = image::ImageBuffer::new(plot.imgx, plot.imgy);
    let black_clr = Color {red: 10, green: 10, blue: 10, alpha: 100};
    let mut curr_clr = &plot.bg_color; 

    for x in 0..plot.imgx {
        for y in 0..plot.imgy {
            let pixel = imgbuf.get_pixel_mut(x, y);
            *pixel = image::Rgb([plot.bg_color.red, plot.bg_color.green, plot.bg_color.blue]);
        }
    }

    // Lines
    for x in 0..plot.imgx {
        for y in 0..plot.imgy {
            if x > 150 && x < 155 {
                curr_clr = &black_clr;
            }
            else if y > 600 && y < 605 {
                curr_clr = &black_clr;
            }
            else {curr_clr = &plot.bg_color;}
            let pixel = imgbuf.get_pixel_mut(x, y);
            *pixel = image::Rgb([curr_clr.red, curr_clr.green, curr_clr.blue]);
        }
    }
    imgbuf
}

pub fn draw_data_points(vec_x: &Vec<i32>, vec_y: &Vec<i32>) {
    if vec_x.len() != vec_y.len() {panic!("Error: Length of Vector X and Y are not the same!");}

    let mut imgbuf = create_plot();
    let b_clr = Color {red: 10, green: 10, blue: 255, alpha: 50};

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
