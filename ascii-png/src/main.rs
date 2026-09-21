use image::GenericImageView;
use std;

//modules (extra files of logic)
mod picture_to_ascii;

fn main() {
    //parse command line arguments
    //1) path to picture
    //2) desired output width
    //3) desired output height
    // for example to run use cargo run -- ../test.jpg 480 200
    let args: Vec<String> = std::env::args().collect();

    //TODO: add checks for the variables to make sure they are correct
    let path = &args[1];
    let width = args[2].parse::<u32>().unwrap(); //crazy rust parsing syntax
    let height = args[3].parse::<u32>().unwrap();

    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open(path).unwrap();

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());

    //convert image to ascii with desired dimensions
    let output_vector = picture_to_ascii::picture_to_ascii(img, width, height);
    
    //write the output out to a text file so we can inspect the work!
    std::fs::write("ascii-art.txt", output_vector).unwrap();
}
