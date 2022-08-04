use picolo::plot::{plot_tbl, PlotSettings, Custom};
use picolo::readimg::*;
mod tests;
mod readimg;

// Example usage
fn main() {

    let a_setting = PlotSettings::default();

    let x = vec![0, 500, 200, 300];
    let y = vec![0, 100, 200, 300];
    plot_tbl(&x, &y, &a_setting, "plot.png");

    let mut b_setting = PlotSettings::default();

    let x = vec![0, 500, 200, 300];
    let y = vec![0, 100, 200, 300];
    plot_tbl(&x, &y, &b_setting, "plot2.png");

    // Color has to be 0-255
    let c_setting = PlotSettings::custom_plot(255, 0, 0);

    let x = vec![0, 100, 200, 300];
    let y = vec![0, 100, 200, 300];
    plot_tbl(&x, &y, &c_setting, "plot3.png");

    // load_picture("plot.png", 100);
    // split_img("plot.png", &100, &100);
}


// Internal todo:
// Make shrink vec function use biggest absolute value as reference instead of a hard coded value
// Automatic way of detecting center of square in case user changes size (currently hardcoded and subtracts by 4). [Easy]
// Draw circles instead of squares (Medium)
// Plot functions like X² (Hard)
// Basic vector manipulation to better fit the graph (normalizing)
// heatmap for projecting a third dimension on current plot_tbl()