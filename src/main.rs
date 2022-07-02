use picolo::plot::plot_tbl;
use picolo::readimg::*;

mod tests;


// Current bug to fix: If Y2 = 500 and Y3 = 501 then it'll cause subtraction by u32
fn main() {
    let x = vec![0, 152, 142, 500, 50, 566];
    let y = vec![0, 152, 100, 600, 50, 900];
    plot_tbl(&x, &y);
}