# Picolo
A very simple Image-reading crate for plotting data (Inspired by Google's simple approach to plotting) and transposing an image into a broken down data structure with the precision decided by the user. All is Very, very alpha-stage right now.

Plot Example: https://imgur.com/a/drPB8e0

# Basic plot

```rust

// Plotting a 2d Vector
// @Params: x: &u32, y: &u32
use picolo::plot::plot_tbl;
fn main() {
    let x = vec![0, 152, 142, 500, 50, 169];
    let y = vec![0, 152, 100, 600, 50, 152];
    plot_tbl(&x, &y);
}
```

# Setting up image table

```rust

// Easiest way to print half the contents
let pixl_struct = load_picture("images/icon.png", 50);

for i in pixl_struct {
     println!("{:?}", i); 
}

// Accessing all fields:
// @Params: &path as &str, precision as u32 (1 = 100% = pixels counted, 2 = 50%, ...)  
use picolo::readimg::load_picture;

let foo = "images/icon.png"; 
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
