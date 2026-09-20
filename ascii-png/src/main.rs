fn main() {
    use std::io::Cursor;
    use image::GenericImageView;

    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open("/root/ASCII-PNG/ascii-png/src/test.jpg").unwrap();

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());

    //turn image grayscale
    let grayscale = img.grayscale();

    //kernel horizontal
    let kernel_horizontal: [f32; 9] = [
        -1.0, -1.0, -1.0, 
        0.0, 0.0, 0.0, 
        1.0, 1.0, 1.0
    ];

    //kernel vertical
    let kernel_vertical: [f32; 9] = [
        -1.0, 0.0, 1.0, 
        -1.0, 0.0, 1.0, 
        -1.0, 0.0, 1.0];

    //edge detected image
    let mut edgy = grayscale.filter3x3(&kernel_horizontal);

    edgy.save("/root/ASCII-PNG/ascii-png/src/edge_test.jpg").unwrap();
    
    edgy.invert();

    edgy.save("/root/ASCII-PNG/ascii-png/src/inverted_edge.jpg").unwrap();
}
