use image::{GenericImageView, ImageBuffer, RgbImage, imageops};
use vector2d::Vector2D;
use rand::Rng;

// Plot.rs is a plotting lib that generates an 800x800 plot based on X and Y Vectors

const IMAGE_X: u32 = 800;
const IMAGE_Y: u32 = 800;

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
pub fn create_plot() {
    
    let mut plot = Plot::default();

    let mut imgbuf = image::ImageBuffer::new(plot.imgx, plot.imgy);

    let black_clr = Color {red: 10, green: 10, blue: 10, alpha: 100};

    let mut curr_clr = &plot.bg_color; 

    // let map = get_key_points(&plot);

    // Canvas
    for x in 0..plot.imgx {
        for y in 0..plot.imgy {
            let pixel = imgbuf.get_pixel_mut(x, y);
            *pixel = image::Rgb([plot.bg_color.red, plot.bg_color.green, plot.bg_color.blue]);
        }
    }

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

    imgbuf.save("plot.png").unwrap();
}


// fn get_key_points(_plot: &Plot) -> Vec<[Pixel; 1]> {

//     let mut plot = _plot;
//     let mut plt_map = Vec::new();
//     let 

//     for i in 0..(plot.imgx / 400) {
//         for j in 0..(plot.imgy / 400) {
//             let color = [
//                 Color {
//                     red: 10,
//                     green: 10,
//                     blue: 10,
//                     alpha: 10,
//                 }
//             ];
//             plt
//         }
//     }
//     plt_map
// }


// 
// fn get_square_ins(_plot: &Plot, _x: &u32, _y: &u32) -> Color {
    
//     let blue_clr = Color {red: 10, green: 10, blue: 255, alpha: 100};
//     let black_clr = Color {red: 8, green: 10, blue: 12, alpha: 100};
    
//     let mut rng = rand::thread_rng();

//     let r = rng.gen_range(1..50);
//     let g = rng.gen_range(1..5);
//     let b = rng.gen_range(1..5);

//     let ret = Color{red: r, green: g, blue: b, alpha: 100};
//     ret
// } 