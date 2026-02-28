use image::{ImageBuffer, Rgb};

fn main() {
    let width = 256;
    let height = 256;

    // Create a new RGB image buffer
    let mut imgbuf = ImageBuffer::new(width, height);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = Rgb([r, 128, b]); // Set pixel color
    }

    // Save the image
    imgbuf.save("generated.png").unwrap();
    println!("Image saved as generated.png");
}
