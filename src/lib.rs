//! Implementations
//!  
//! Plotting a 2d Vector
//! @Params: x: &u32, y: &u32 (Make sure Vector X and Vector Y have the same number of elements)
//! ```
//! use picolo::plot::plot_tbl;
//! fn main() {
//!     let x = vec![1, -1512, 30, 500, 50, 1619];
//!     let y = vec![20, -1212, -1512, 12541, 5215, 602];
//!     plot_tbl(&x, &y);
//! }
//! ```
//! Easiest way to print half the contents
//!
//! ```
//! let pixl_struct = load_picture("icon.png", 2);
//!
//! for i in pixl_struct {
//!      println!("{:?}", i);
//! }
//! ```
//! Accessing all fields:
//! @Params: path as &str, precision as u32 (1 = 100% precision, 2 = 50% ...)  
//! ```
//! let foo = "icon.png";
//! let bar_str = &foo;
//! let precision = 1;
//! let pixl_struct = picolo::load_picture(bar_str, precision);
//!
//! //Print all the contents derived from image
//! for i in pixl_struct {
//!    println!("x {} y {} red {} green {} blue: {}", i[0].x, i[0].y,
//!    i[0].color.red, i[0].color.green, i[0].color.blue);
//! }
//!
//! ```
pub mod listops;
pub mod plot;
pub mod readimg;
