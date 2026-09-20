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

    grayscale.save("/root/ASCII-PNG/ascii-png/src/grayscale_test.jpg").unwrap();
}
