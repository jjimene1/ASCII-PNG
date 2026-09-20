use image::GenericImageView;
use image::Rgba;
use std;

fn main() {


    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open("/root/ASCII-PNG/ascii-png/src/test.jpg").unwrap();

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());

    //turn image grayscale
    let grayscale: image::DynamicImage = img.grayscale();

    //sharpening kernel
    let kernel_sharpening: [f32; 9] = [
        0.0, -1.0, 0.0,
       -1.0, 5.0, -1.0,
        0.0, -1.0, 0.0
    ];

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

    //edge detection with simple horizontal and vertical filters
    let first_pass = grayscale.filter3x3(&kernel_sharpening);

    let second_pass = first_pass.filter3x3(&kernel_vertical);

    let third_pass = second_pass.filter3x3(&kernel_horizontal);

    third_pass.save("/root/ASCII-PNG/ascii-png/src/edge_test.jpg").unwrap();

    let ascii_char: [char; 11] = ['.', ',', ':', ';', '+', '*', '?', '%', 'S', '#', '@'];

    //to print to console or better see the image we need to resize it 
    let nwidth = 480;
    let nheight = 200;
    let grayscale_small = grayscale.resize(nwidth, nheight, image::imageops::FilterType::Gaussian); //maintains aspect ratio
    grayscale_small.save("/root/ASCII-PNG/ascii-png/src/resize_test.jpg").unwrap();

    let x = grayscale_small.width();
    let y = grayscale_small.height();

    //create a string vector for our ascii art
    let mut output = String::new();

    // Let's take a simple approach for the ascii and just divide each region of the picture according to it's pixel brightness and fill it in
    // with the characters from the above array using the following formula: c_index = [(Brightness)(num_char - 1)]
    for i in 0..y {
        for f in 0..x {
            let pixel = grayscale_small.get_pixel(f, i); //get pixel
            let brightness = calculate_brightness(pixel); //calculate brightness
            let index_ascii = ((brightness) * ((ascii_char.len() - 1) as f32)) as usize; //calculate index for ascii array
            let character = ascii_char[index_ascii];
            // now we print the character to the terminal
            print!("{}", character);
            output.push(character);
        }
        println!(); //new line when we reach the end of the row
        output.push('\n');
    }

    //write the output out to a text file so we can inspect the work!
    std::fs::write("ascii-art.txt", output).unwrap();
}

//helper function for calculating the brightness of pixels by taking their perceived luminance returned as a floating point from 0.0 to 1.0
fn calculate_brightness (pixel: Rgba<u8>) -> f32 {
    //rgba format from image is a weird data struct so we decompose the array and normalize the values

    let mut normalized_rgb: [f32; 3] = [0.0; 3]; //initialize array size 3 with 0
    for (index, channel) in normalized_rgb.iter_mut().enumerate() { //return the mutable elements of the array + the index
        *channel = (pixel[index] as f32) / 255.0; //use index to decompose rgba and dereference to change array variables
    }

    //we are ready to calculate brightness usinf the weighted sum!
    let brightness = 0.2126 * normalized_rgb[0] + 0.7152 * normalized_rgb[1] + 0.0722 * normalized_rgb[2]; 
    
    return brightness;
}
