use picolo::plot::*;

// Simple example on how to plot a datapoint array
fn main() {
    let x = vec![0, 22, 113];
    let y = vec![0, 152, 52];
    draw_data_points(&x, &y);
}

