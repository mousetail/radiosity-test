use crate::export_gltf::{save_mesh, SaveMeshError};
use crate::vector::{Vec2, Vec3};

struct CubeSides {
    edge: u8,
    front: bool,
}

impl CubeSides {
    fn new() -> Self {
        CubeSides {
            edge: 0,
            front: false,
        }
    }

    fn get_item(&self) -> CubeSide {
        let normal: (Vec3, Vec3, Vec3) = match self.edge {
            0 => (Vec3 { x: 1., y: 0., z: 0. }, Vec3 { x: 0., y: 1., z: 0. }, Vec3 { x: 0., y: 0., z: 1. }),
            1 => (Vec3 { x: 0., y: 1., z: 0. }, Vec3 { x: 0., y: 0., z: 1. }, Vec3 { x: 1., y: 0., z: 0. }),
            2 => (Vec3 { x: 0., y: 0., z: 1. }, Vec3 { x: 1., y: 0., z: 0. }, Vec3 { x: 0., y: 1., z: 0. }),
            _ => panic!("Invalid edge")
        };

        let mut positions: [Vec3; 4] = [
            Vec3 { x: 0., y: 0., z: 0. },
            normal.1,
            normal.2,
            normal.1 + normal.2
        ];

        if self.front {
            for i in 0..positions.len() {
                positions[i] += normal.0
            }
        }

        let indices = if self.front {
            [0, 1, 2, 2, 1, 3]
        } else {
            [0, 2, 1, 1, 2, 3]
        };

        let offset_multiplier: i8 = if (self.front) { 1 } else { -1 };
        let offset = (normal.0.x as i8 * offset_multiplier, normal.0.y as i8 * offset_multiplier, normal.0.z as i8 * offset_multiplier);

        CubeSide {
            vertices: positions,
            normals: [normal.0; 4],
            indices,
            offset,
        }
    }
}

struct CubeSide {
    pub vertices: [Vec3; 4],
    pub normals: [Vec3; 4],
    pub indices: [usize; 6],
    pub offset: (i8, i8, i8),
}

impl Iterator for CubeSides {
    type Item = CubeSide;

    fn next(&mut self) -> Option<Self::Item> {
        if self.edge == 3 {
            return Option::None;
        }
        let item: CubeSide = self.get_item();

        if self.front {
            self.edge += 1;
            self.front = false;
        } else {
            self.front = true;
        }
        return Option::Some(item);
    }
}

fn is_empty_or_out_of_bounds<const SIZE: usize>(voxels: &[[[u16; SIZE]; SIZE]; SIZE], coords: (i32, i32, i32)) -> bool {
    if coords.0 < 0 || coords.1 < 0 || coords.2 < 0 {
        return true;
    }
    if coords.0 >= SIZE as i32 || coords.1 >= SIZE as i32 || coords.2 >= SIZE as i32 {
        return true;
    }
    return voxels[coords.0 as usize][coords.1 as usize][coords.2 as usize] == 0;
}

pub fn voxel_to_mesh<const SIZE: usize>(voxels: [[[u16; SIZE]; SIZE]; SIZE], filename: String) -> Result<(), SaveMeshError> {
    let mut positions: Vec<Vec3> = Vec::new();
    let mut normals: Vec<Vec3> = Vec::new();
    let mut texture_coordinates: Vec<Vec2> = Vec::new();
    let mut indexes: Vec<usize> = Vec::new();

    for x in 0..SIZE {
        for y in 0..SIZE {
            for z in 0..SIZE {
                if voxels[x][y][z] != 0 {
                    let base_position = Vec3 {
                        x: x as f32 / SIZE as f32,
                        y: y as f32 / SIZE as f32,
                        z: z as f32 / SIZE as f32,
                    };

                    for cube in CubeSides::new() {
                        if is_empty_or_out_of_bounds(&voxels, (
                            (cube.offset.0) as i32 + x as i32,
                            (cube.offset.1) as i32 + y as i32,
                            (cube.offset.2) as i32 + z as i32
                        )) {
                            let length = positions.len();
                            positions.extend(cube.vertices.map(|x| x * (1.0 / SIZE as f32) + base_position));
                            normals.extend(cube.normals);
                            indexes.extend(cube.indices.map(|x| x + length));
                            texture_coordinates.extend([
                                Vec2 { x: 0., y: 0. },
                                Vec2 { x: 1., y: 0. },
                                Vec2 { x: 0., y: 1. },
                                Vec2 { x: 1., y: 1. },
                            ])
                        }
                    }
                }
            }
        }
    }

    save_mesh(
        filename,
        &positions,
        &normals,
        &texture_coordinates,
        &indexes,
    )
}
