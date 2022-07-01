# Picolo
Image-reading crate for transposing an image into its bareboned data. 

Input(Image) -> Output(Pixel: x, y, color[r, g, b, a])

# Implementation

```rust
// Name of image path. In this case it's in a folder called images next to src
let foo = "images/ig-icon.png"; 
let bar_str = &foo;
let pixl_struct = picolo::load_picture(bar_str);

// Print all the contents derived from image
for i in pixl_struct {
    println!("x {} y {} red {} green {} blue: {}", i[0].x, i[0].y, 
    i[0].color.red, i[0].color.green, i[0].color.blue);
}
```

#TODO:
* Implement shrinking if dataset is too big (Idea: check the biggest number in a list, check if bigger than bounds, and divide by the difference of list surpassing the bounds eg: if bounds = 1000 and an element in list is 1100 then divide by 10%)
*Convert to uf32 functionality (Floats are just easier to work with)
* Add option to draw circles if preferred
* Add coloring variation if dataset become too large for the plot
* Add line drawing
* Add function to open image after the plot has been drawn so the user doesn't have to
* Refactor current way of finding origin with exact positions