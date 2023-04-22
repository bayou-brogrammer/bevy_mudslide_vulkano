/// Converts array of 4 u8 colors to u32
pub fn u8_rgba_to_u32_rgba(r: u8, g: u8, b: u8, a: u8) -> u32 {
    ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | (a as u32)
}

pub fn u32_rgba_to_u32_abgr(num: u32) -> u32 {
    let r = num & 255;
    let g = (num >> 8) & 255;
    let b = (num >> 16) & 255;
    let a = (num >> 24) & 255;
    (r << 24) | (g << 16) | (b << 8) | a
}

/// Converts u32 color to array of 4 u8
pub fn u32_rgba_to_u8_rgba(num: u32) -> [u8; 4] {
    let r = num & 255;
    let g = (num >> 8) & 255;
    let b = (num >> 16) & 255;
    let a = (num >> 24) & 255;
    [a as u8, b as u8, g as u8, r as u8]
}

pub fn u32_rgba_to_f32_rgba(num: u32) -> [f32; 4] {
    let color_u8 = u32_rgba_to_u8_rgba(num);
    [
        color_u8[0] as f32 / 255.0,
        color_u8[1] as f32 / 255.0,
        color_u8[2] as f32 / 255.0,
        color_u8[3] as f32 / 255.0,
    ]
}

pub fn color_rgba_f32(rgba: [u8; 4]) -> [f32; 4] {
    [
        rgba[0] as f32 / 255.0,
        rgba[1] as f32 / 255.0,
        rgba[2] as f32 / 255.0,
        rgba[3] as f32 / 255.0,
    ]
}

pub fn variated_color(color: [u8; 4]) -> [u8; 4] {
    use rand::Rng;

    let p = rand::thread_rng().gen::<f32>();
    let r = color[0] as f32 / 255.0;
    let g = color[1] as f32 / 255.0;
    let b = color[2] as f32 / 255.0;
    let variation = -0.1 + 0.2 * p;

    let r = ((r + variation).clamp(0.0, 1.0) * 255.0) as u8;
    let g = ((g + variation).clamp(0.0, 1.0) * 255.0) as u8;
    let b = ((b + variation).clamp(0.0, 1.0) * 255.0) as u8;
    let a = color[3];

    [r, g, b, a]
}
