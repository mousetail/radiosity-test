use image::Rgba;

#[derive(Copy, Clone, Debug)]
pub struct RadiosityColor {
    pub color: Rgba<u8>,
    pub emission: f32
}
