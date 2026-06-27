use base64::{engine::general_purpose::STANDARD, Engine};
use image::{Rgb, RgbImage};
use qrcode::QrCode;

#[tauri::command]
pub fn generate_qr(text: String) -> Result<String, String> {
    let code = QrCode::new(text.as_bytes()).map_err(|e| e.to_string())?;
    let luma_image = code
        .render::<image::Luma<u8>>()
        .max_dimensions(400, 400)
        .quiet_zone(true)
        .build();

    // Convert to RGB with custom colors: dark=#0f172a, light=#ffffff
    let width = luma_image.width();
    let height = luma_image.height();
    let mut rgb_image = RgbImage::new(width, height);
    for (x, y, pixel) in luma_image.enumerate_pixels() {
        let color = if pixel[0] == 0 {
            Rgb([0x0fu8, 0x17u8, 0x2au8])
        } else {
            Rgb([0xffu8, 0xffu8, 0xffu8])
        };
        rgb_image.put_pixel(x, y, color);
    }

    let mut png_bytes: Vec<u8> = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut png_bytes);
    rgb_image
        .write_to(&mut cursor, image::ImageFormat::Png)
        .map_err(|e| e.to_string())?;

    let encoded = STANDARD.encode(&png_bytes);
    Ok(format!("data:image/png;base64,{}", encoded))
}
