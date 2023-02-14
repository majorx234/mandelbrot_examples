use image::save_buffer_with_format;
use image::ColorType;
use image::ImageError;

pub fn write_png(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), ImageError> {
    image::save_buffer_with_format(
        filename,
        pixels,
        bounds.0 as u32,
        bounds.1 as u32,
        ColorType::L8,
        //image::ColorType::Rgb8
        image::ImageFormat::Png,
    )?;
    Ok(())
}
