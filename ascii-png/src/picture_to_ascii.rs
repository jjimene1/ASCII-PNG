use image::GenericImageView;
use image::Rgba;

//TODO: Add downscale filter as an input to this function?
//might be helpful if we are messing around with things later

//Takes in loaded picture from image library and converts to ascii!
//returns string vector array with picture, also prints the ascii to console
pub fn picture_to_ascii (img: image::DynamicImage, width: u32, height: u32) -> String { 
    //first convert image to grayscale
    let grayscale = img.grayscale();

    //downscale image to requested dimensions while preserving aspect ratio of OG image
    let downscale_img =  grayscale.resize(width, height, image::imageops::FilterType::Gaussian);

    //determine downscale dimensions
    let x = downscale_img.width();
    let y = downscale_img.height();

    //create a string vector for ascii art
    let mut output: String = String::new();

    //define ascii table for conversion
    let ascii_char: [char; 11] = ['.', ',', ':', ';', '+', '*', '?', '%', 'S', '#', '@'];

    // Let's take a simple approach for the ascii and just divide each region of the picture according to it's pixel brightness and fill it in
    // with the characters from the above array using the following formula: c_index = [(Brightness)(num_char - 1)]
    for i in 0..y {
        for f in 0..x {
            let pixel = downscale_img.get_pixel(f, i); //get pixel
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

    return output;
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