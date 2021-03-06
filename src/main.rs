use picolo::plot::plot_tbl;
use picolo::readimg::*;
mod tests;
mod readimg;


// main.rs is primarily used for testing purposes- it has nothing to do with the crate itself for the end-user

// Example usage
fn main() {
    let x = vec![-0, -151, -0, -500, 50, -8612529];
    let y = vec![0, 12152, 12521, 254120, 521550, 602];
    plot_tbl(&x, &y);

    // load_picture("plot.png", 100);
    // split_img("plot.png", &100, &100);
}

