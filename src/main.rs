use picolo::plot::*;

// Simple example on how to plot a datapoint array
fn main() {
    let x = vec![0, 22, 800];
    let y = vec![0, 152, 800];
    draw_data_points(&x, &y);
}