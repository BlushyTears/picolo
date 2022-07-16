use picolo::plot::plot_tbl;
use picolo::readimg::*;
use picolo::split::split_img;
mod tests;
mod readimg;

// Example usage
fn main() {
    let x = vec![1, 1512, 30, 500, 50, 1619];
    let y = vec![20, 1212, 1512, 12541, 5215, 602];
    plot_tbl(&x, &y);

    // load_picture("plot.png", 100);
    // split_img("plot.png", &100, &100);
}

