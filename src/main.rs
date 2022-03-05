use image::{Rgb, RgbImage};
use crate::voxel::voxel_to_mesh;
use image::io::Reader as ImageReader;
use crate::image_to_grid::{COLORS, left_curve_segment, right_curve_segment, straight_segment};

mod export_gltf;
mod vector;
mod voxel;
mod image_to_grid;
mod radiosity;
mod radiosity_color;

fn main() {
    let mut texture = RgbImage::new(16, 16);
    for (index, color) in COLORS.iter().enumerate() {
        texture.put_pixel(
            (index / 16) as u32,
            (index % 16) as u32,
            Rgb(*color),
        )
    }
    texture.save("media/colormap.png").unwrap();

    let image = ImageReader::open("media/hallway_edge.png").unwrap().decode().unwrap().into_rgba8();
    let end_image = ImageReader::open("media/hallway_edge_end.png").unwrap().decode().unwrap().into_rgba8();
    let layers = [
        end_image.clone(),
        image.clone(),
        image.clone(),
        image.clone(),
        image.clone(),
        image.clone(),
        image.clone(),
        image.clone(),
        image.clone(),
        image.clone(),
        image.clone(),
        image.clone(),
        image.clone(),
        image.clone(),
        image.clone(),
        end_image
    ];

    let room_image = ImageReader::open("media/room_edge.png").unwrap().decode().unwrap().into_rgba8();
    let room_end_image = ImageReader::open("media/room_edge_end.png").unwrap().decode().unwrap().into_rgba8();

    let room_layers = [
        room_end_image.clone(),
        room_image.clone(),
        room_image.clone(),
        room_image.clone(),
        room_image.clone(),
        room_image.clone(),
        room_image.clone(),
        room_image.clone(),
        room_image.clone(),
        room_image.clone(),
        room_image.clone(),
        room_image.clone(),
        room_image.clone(),
        room_image.clone(),
        room_image.clone(),
        room_end_image
    ];

    voxel_to_mesh(straight_segment(&layers
    ), "hallway".to_string()).unwrap();
    voxel_to_mesh(
        left_curve_segment(&layers),
        "hallway_curve_left".to_string()).unwrap();
    voxel_to_mesh(
        right_curve_segment(&layers),
        "hallway_curve_right".to_string()).unwrap();

    voxel_to_mesh(straight_segment(&room_layers
    ), "room".to_string()).unwrap();
    voxel_to_mesh(
        left_curve_segment(&room_layers),
        "room_curve_left".to_string()).unwrap();
    voxel_to_mesh(
        right_curve_segment(&room_layers),
        "room_curve_right".to_string()).unwrap();
}
