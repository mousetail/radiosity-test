use image::{RgbaImage, Rgb, Rgba};
use crate::radiosity_color::RadiosityColor;

pub const COLORS: [[u8; 3]; 256] = [
    [0, 0, 0], [128, 0, 0], [0, 128, 0], [128, 128, 0], [0, 0, 128], [128, 0, 128], [0, 128, 128], [192, 192, 192], [128, 128, 128], [255, 0, 0], [0, 255, 0], [255, 255, 0], [0, 0, 255], [255, 0, 255], [0, 255, 255], [255, 255, 255], [0, 0, 0], [0, 0, 95], [0, 0, 135], [0, 0, 175], [0, 0, 215], [0, 0, 255], [0, 95, 0], [0, 95, 95], [0, 95, 135], [0, 95, 175], [0, 95, 215], [0, 95, 255], [0, 135, 0], [0, 135, 95], [0, 135, 135], [0, 135, 175], [0, 135, 215], [0, 135, 255], [0, 175, 0], [0, 175, 95], [0, 175, 135], [0, 175, 175], [0, 175, 215], [0, 175, 255], [0, 215, 0], [0, 215, 95], [0, 215, 135], [0, 215, 175], [0, 215, 215], [0, 215, 255], [0, 255, 0], [0, 255, 95], [0, 255, 135], [0, 255, 175], [0, 255, 215], [0, 255, 255], [95, 0, 0], [95, 0, 95], [95, 0, 135], [95, 0, 175], [95, 0, 215], [95, 0, 255], [95, 95, 0], [95, 95, 95], [95, 95, 135], [95, 95, 175], [95, 95, 215], [95, 95, 255], [95, 135, 0], [95, 135, 95], [95, 135, 135], [95, 135, 175], [95, 135, 215], [95, 135, 255], [95, 175, 0], [95, 175, 95], [95, 175, 135], [95, 175, 175], [95, 175, 215], [95, 175, 255], [95, 215, 0], [95, 215, 95], [95, 215, 135], [95, 215, 175], [95, 215, 215], [95, 215, 255], [95, 255, 0], [95, 255, 95], [95, 255, 135], [95, 255, 175], [95, 255, 215], [95, 255, 255], [135, 0, 0], [135, 0, 95], [135, 0, 135], [135, 0, 175], [135, 0, 215], [135, 0, 255], [135, 95, 0], [135, 95, 95], [135, 95, 135], [135, 95, 175], [135, 95, 215], [135, 95, 255], [135, 135, 0], [135, 135, 95], [135, 135, 135], [135, 135, 175], [135, 135, 215], [135, 135, 255], [135, 175, 0], [135, 175, 95], [135, 175, 135], [135, 175, 175], [135, 175, 215], [135, 175, 255], [135, 215, 0], [135, 215, 95], [135, 215, 135], [135, 215, 175], [135, 215, 215], [135, 215, 255], [135, 255, 0], [135, 255, 95], [135, 255, 135], [135, 255, 175], [135, 255, 215], [135, 255, 255], [175, 0, 0], [175, 0, 95], [175, 0, 135], [175, 0, 175], [175, 0, 215], [175, 0, 255], [175, 95, 0], [175, 95, 95], [175, 95, 135], [175, 95, 175], [175, 95, 215], [175, 95, 255], [175, 135, 0], [175, 135, 95], [175, 135, 135], [175, 135, 175], [175, 135, 215], [175, 135, 255], [175, 175, 0], [175, 175, 95], [175, 175, 135], [175, 175, 175], [175, 175, 215], [175, 175, 255], [175, 215, 0], [175, 215, 95], [175, 215, 135], [175, 215, 175], [175, 215, 215], [175, 215, 255], [175, 255, 0], [175, 255, 95], [175, 255, 135], [175, 255, 175], [175, 255, 215], [175, 255, 255], [215, 0, 0], [215, 0, 95], [215, 0, 135], [215, 0, 175], [215, 0, 215], [215, 0, 255], [215, 95, 0], [215, 95, 95], [215, 95, 135], [215, 95, 175], [215, 95, 215], [215, 95, 255], [215, 135, 0], [215, 135, 95], [215, 135, 135], [215, 135, 175], [215, 135, 215], [215, 135, 255], [215, 175, 0], [215, 175, 95], [215, 175, 135], [215, 175, 175], [215, 175, 215], [215, 175, 255], [215, 215, 0], [215, 215, 95], [215, 215, 135], [215, 215, 175], [215, 215, 215], [215, 215, 255], [215, 255, 0], [215, 255, 95], [215, 255, 135], [215, 255, 175], [215, 255, 215], [215, 255, 255], [255, 0, 0], [255, 0, 95], [255, 0, 135], [255, 0, 175], [255, 0, 215], [255, 0, 255], [255, 95, 0], [255, 95, 95], [255, 95, 135], [255, 95, 175], [255, 95, 215], [255, 95, 255], [255, 135, 0], [255, 135, 95], [255, 135, 135], [255, 135, 175], [255, 135, 215], [255, 135, 255], [255, 175, 0], [255, 175, 95], [255, 175, 135], [255, 175, 175], [255, 175, 215], [255, 175, 255], [255, 215, 0], [255, 215, 95], [255, 215, 135], [255, 215, 175], [255, 215, 215], [255, 215, 255], [255, 255, 0], [255, 255, 95], [255, 255, 135], [255, 255, 175], [255, 255, 215], [255, 255, 255], [8, 8, 8], [18, 18, 18], [28, 28, 28], [38, 38, 38], [48, 48, 48], [58, 58, 58], [68, 68, 68], [78, 78, 78], [88, 88, 88], [98, 98, 98], [108, 108, 108], [118, 118, 118], [128, 128, 128], [138, 138, 138], [148, 148, 148], [158, 158, 158], [168, 168, 168], [178, 178, 178], [188, 188, 188], [198, 198, 198], [208, 208, 208], [218, 218, 218], [228, 228, 228], [238, 238, 238]
];

fn closest_color(color: &Rgba<u8>) -> u8 {
    let mut min_distance = 256 * 3;
    let mut min_index = 0;
    for (index, compare_color) in COLORS.iter().enumerate() {
        let distance = (compare_color[0] as i32 - color[0] as i32).abs() + (compare_color[1] as i32 - color[1] as i32).abs() + (compare_color[2] as i32 - color[2] as i32).abs();
        if distance < min_distance {
            min_index = index;
            min_distance = distance;
        }
    }
    return min_index as u8;
}



fn curve_segment(layers: &[RgbaImage; 16], cmp: fn(x: usize, z: usize)->bool, brightness: f32) -> [[[RadiosityColor; 16]; 16]; 16] {
    let mut grid = [[[RadiosityColor {color: [0, 0, 0, 255].into(), emission: 0.}; 16]; 16]; 16];
    for x in 0..16 {
        for y in 0..16 {
            for z in 0..16 {
                let color = if cmp(x, z) {layers[x].get_pixel(z as u32, layers[x].height() - 1 - y as u32)} else {
                    layers[z].get_pixel(x as u32, layers[x].height() - 1 - y as u32)
                };
                grid[x][y][z].color = *color;
                grid[x][y][z].emission = if color.0 == [255, 255, 255, 255] { brightness } else {0. };
            }
        }
    };
    grid
}

pub fn straight_segment(layers: &[RgbaImage; 16], brightness: f32) -> [[[RadiosityColor; 16]; 16]; 16] {
    return curve_segment(layers, |x, z| true, brightness)
}

pub fn left_curve_segment(layers: &[RgbaImage; 16], brightness: f32) -> [[[RadiosityColor; 16]; 16]; 16] {
    return curve_segment(layers, |x, z| x < z, brightness)
}

pub fn right_curve_segment(layers: &[RgbaImage; 16], brightness: f32) -> [[[RadiosityColor; 16]; 16]; 16] {
    return curve_segment(layers, |x, z| x > z, brightness)
}
