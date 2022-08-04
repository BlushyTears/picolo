# Picolo
A simple, yet comprehensive Image-reading crate for plotting data with dynamically placed origin based on dataset. (Inspired by Google's simple approach to plotting) and an image reading crate that breaks down data with the precision decided by the user. All is Very, very alpha-stage right now.

Plot Example: https://imgur.com/a/i72KTgB

# Basic plot

```rust

// Plotting a 2d Vector with log scale in mind
// @Params: x: &u32, y: &u32
use picolo::plot::*;
fn main() {
    let a_setting = PlotSettings::default();

    let x = vec![0, 500, 200, 300];
    let y = vec![0, 100, 200, 300];
    plot_tbl(&x, &y, &a_setting);
}
```

# Setting up image table

```rust

// Easiest way to print half the contents
let pixl_struct = load_picture("icon.png", 50);

for i in pixl_struct {
     println!("{:?}", i); 
}

// Accessing all fields:
// @Params: &path as &str, precision as u32 (100 = 100% = pixels counted, 50 = 50%, ...)  
use picolo::readimg::load_picture;

let foo = "icon.png"; 
let bar_str = &foo;
let precision = 100;
let pixl_struct = picolo::load_picture(bar_str, precision);

// Print all the contents derived from image
for i in pixl_struct {
    println!("x {} y {} red {} green {} blue: {}", i[0].x, i[0].y, 
    i[0].color.red, i[0].color.green, i[0].color.blue);
}

```

#TODO:
* Add option to draw circles if preferred (.json file preferably)
* Add coloring variation if dataset become too large for the plot
* Add text above individual datapoints (Display: as '50, 12')
* Add line drawing
* Add function to open image after the plot has been drawn so the user doesn't have to
* Refactor current way of finding origin with more exact positions
* Fix allignment issues
