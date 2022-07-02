# Picolo
A very simple Image-reading crate for plotting data and transposing image contents into a detailed data structure with the precision decided by the user. Very, very alpha-stage right now.

Precision explanation: Say you only wanted to count every 10 pixels to increase performance, and your AI model doesn't care about individual pixels. Changing the pixel from 1 to 10 will do that, and allow you to compute more data for your buck.

Plot: https://imgur.com/a/drPB8e0

# Basic plot

```rust

// Plotting a 2d Vector
// @Params: x: &u32, y: &u32
use picolo::plot::plot_tbl;
fn main() {
    let x = vec![0, 152, 142, 500, 50, 169];
    let y = vec![0, 152, 100, 600, 50, 152];
    plot(&x, &y);
}
```

# Setting up image transpose

```rust

// Easiest way to print half the contents
let pixl_struct = load_picture("images/icon.png", 2);

for i in pixl_struct {
     println!("{:?}", i); 
}

// Accessing all fields:
// @Params: &path as &str, precision as u32 (1 = 100% precision, 2 = 50%, ...)  
use picolo::readimg::load_picture;

let foo = "images/icon.png"; 
let bar_str = &foo;
let precision = 1;
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
