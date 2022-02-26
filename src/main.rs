use crate::voxel::voxel_to_mesh;

mod export_gltf;
mod vector;
mod voxel;

fn main() {
    voxel_to_mesh([[[0, 1], [1, 1]], [[1, 1]; 2]], "cube".to_string()).unwrap();
}
