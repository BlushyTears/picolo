# picolo
Image-reading crate for transposing an image into its bareboned data. 

Input(Image) -> Output(Pixel: x, y, color[r, g, b, a])

# Implementation (Until crates.io documentation starts to work)

```rust
let foo = "images/ig-icon.png"; // Name of image source, in this case it's in a folder called images
let bar_str = &foo;
let pixl_struct = picolo::load_picture(bar_str);

// Print all the contents derived from image
for i in pixl_struct {
    println!("x {} y {} red {} green {} blue: {}", i[0].x, i[0].y, 
    i[0].color.red, i[0].color.green, i[0].color.blue);
}
```
