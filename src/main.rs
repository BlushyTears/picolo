use picolo::plot::plot;

// Simple example on how to plot a datapoint array
fn main() {
    let x = vec![0, 152, 1000];
    let y = vec![0, 152, 490];
    plot(&x, &y);
}