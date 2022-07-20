use picolo::plot::plot_tbl;
use picolo::readimg::*;
use picolo::split::split_img;
mod tests;
mod readimg;


// main.rs is primarily used for testing purposes- it has nothing to do with the crate itself for the end-user

// Example usage
fn main() {
    let x = vec![0, -152, 10];
    let y = vec![0, -117, 192];
    plot_tbl(&x, &y);

    // load_picture("plot.png", 100);
    // split_img("plot.png", &100, &100);
}

