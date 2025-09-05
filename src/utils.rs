pub fn rgb_to_hex(colorf: [f32; 3]) -> String{
    let color = rgb_f32_to_u8(colorf);
    //return format!("#{:02X}{:02X}{:02X}", color[0], color[1], color[2]);
    return format!("#{}", hex::encode(color));
}
pub fn rgb_u8_to_f32(rgb: [u8; 3]) -> [f32; 3] {
    println!("{},{},{}", rgb[0], rgb[1], rgb[2]);
    [
        (rgb[0] as f32 / 255.0).powf(2.2),
        (rgb[1] as f32 / 255.0).powf(2.2),
        (rgb[2] as f32 / 255.0).powf(2.2),
    ]
}


pub fn rgb_f32_to_u8(rgb: [f32; 3]) -> [u8; 3] {
    [
        (rgb[0].powf(1.0/2.2) * 255.0).round() as u8,
        (rgb[1].powf(1.0/2.2) * 255.0).round() as u8,
        (rgb[2].powf(1.0/2.2) * 255.0).round() as u8,
    ]
}