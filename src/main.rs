use picolo::plot::plot_tbl;
use picolo::readimg::*;
use picolo::split::split_img;
mod tests;
mod readimg;

// Example usage
fn main() {
    // let x = vec![0, 152, 142, 500, 50, 800];
    // let y = vec![0, 152, 100, 600, 50, 800];
    // plot_tbl(&x, &y);

    // load_picture("plot.png", 100);
    split_img("plot.png", &100, &100);
}

