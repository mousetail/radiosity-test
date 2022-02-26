use json::{JsonError, object, array};
use std::fs::File;
use std::io::Write;
use std::{fs, result};
use crate::vector::{Vec2, Vec3};
use thiserror::{Error};

fn float_max<T>(it: T) -> f32 where T: Iterator<Item=f32> {
    it.fold(-1. / 0., f32::max)
}

fn float_min<T>(it: T) -> f32 where T: Iterator<Item=f32> {
    it.fold(
        1. / 0., f32::min,
    )
}

#[derive(Debug, Error)]
pub enum SaveMeshError {
    #[error("Sjon Error")]
    JsonError(#[from] JsonError),
    #[error("IO Error")]
    IOError(#[from] std::io::Error)
}

pub fn save_mesh(
    filename: String,
    vertexes: &Vec<Vec3>,
    normals: &Vec<Vec3>,
    uvs: &Vec<Vec2>,
    indices: &Vec<usize>,
) -> result::Result<(), SaveMeshError> {
    let min_vertex = [
        float_min(vertexes.iter().map(|i| i.x)),
        float_min(vertexes.iter().map(|i| i.y)),
        float_min(vertexes.iter().map(|i| i.z)),
    ];
    let max_vertex = [
        float_max(vertexes.iter().map(|i| i.x)),
        float_max(vertexes.iter().map(|i| i.y)),
        float_max(vertexes.iter().map(|i| i.z)),
    ];


    let gltf_json_part = object! {
        "asset"=> object!{
            "generator": "None",
            "version": "2.0"
        },
        "scene"=> 0,
        "scenes"=>array![
            object!{
                "name"=> "Scene0",
                "nodes" => array![0]
            }
        ],
        "nodes"=>array![
            object!{
                "mesh"=>0,
                "name"=>"curve"
            }
        ],
        "meshes"=> array![
            object!{
                "primitives"=> array![
                    object!{
                        "attributes"=>object!{
                            "NORMAL"=> 0,
                            "POSITION"=>1,
						    "TEXCOORD_0"=>2
                        },
                        "indices"=>3
                    }
                ]
            }
        ],
        "accessors"=>array![
            object!{
                "bufferView"=>0,
                "componentType"=> 5126_u32, // Float
                "count"=> normals.len(),
                "type"=> "VEC3"
            },
            object!{
                "bufferView"=>1,
                "componentType"=> 5126_u32, // Float
                "count"=> vertexes.len(),
                "type"=> "VEC3",
                "min"=>array![min_vertex[0], min_vertex[1], min_vertex[2]],
                "max"=>array![max_vertex[0], max_vertex[1], max_vertex[2]],
            },
            object!{
                "bufferView"=>2,
                "componentType"=> 5126_u32, // Float
                "count"=> uvs.len(),
                "type"=> "VEC2"
            },
            object!{
                "bufferView"=>3,
                "componentType"=> 5125_u32, // Unsigned Int
                "count"=> indices.len(),
                "type"=> "SCALAR"
            }
        ],
        "bufferViews"=> array![
            object!{
                "buffer"=>0,
                "byteLength"=>4 *3 * normals.len(),
                "byteOffset"=>0,
            },
            object!{
                "buffer"=>0,
                "byteOffset"=>4 * 3 * normals.len(),
                "byteLength"=>4 * 3 * vertexes.len(),
            },

            object!{
                "buffer"=>0,
                "byteOffset"=>4 * 3 * normals.len() + 4 * 3 * vertexes.len(),
                "byteLength"=>4 * 2 * uvs.len(),
            },
            object!{
                "buffer"=>0,
                "byteOffset"=>4 * 3 * normals.len() + 4 * 3 * vertexes.len() + 4 * 2 * uvs.len(),
                "byteLength"=>4 * indices.len(),
            }
        ],
        "buffers"=>array![
            object!{
                "byteLength"=>4 * 3 * normals.len() + 4 * 3 * vertexes.len() + 4 * 2 * uvs.len() + 4 * indices.len()
            }
        ]
    };

    fs::create_dir_all("cache")?;
    let mut jsfile = File::create(format!("cache/{:}.json", filename)).unwrap();
    jsfile.write_all(
        json::stringify_pretty(gltf_json_part.clone(), 2).as_bytes()
    )?;

    let mut data = json::stringify(gltf_json_part);
    while data.len() % 4 != 0 {
        data += " "
    };


    let buffer_normals: Vec<u8> = normals
        .iter()
        .map(|x| [x.x.to_le_bytes(), x.y.to_le_bytes(), x.z.to_le_bytes()])
        .flatten()
        .flatten()
        .collect();
    let buffer_positions: Vec<u8> = vertexes.iter().map(
        |x| [x.x.to_le_bytes(), x.y.to_le_bytes(), x.z.to_le_bytes()]).flatten().flatten().collect();
    let buffer_uvs: Vec<u8> = uvs.iter().map(|x| [x.x.to_le_bytes(), x.y.to_le_bytes()]).flatten().flatten().collect();
    let buffer_indices: Vec<u8> = indices.iter().map(|x| (*x as u32).to_le_bytes()).flatten().collect();

    let mut file = File::create(format!("cache/{:}.glb", filename)).unwrap();
    file.write_all("glTF".as_bytes())?;
    file.write_all(&2_u32.to_le_bytes())?;
    file.write_all(
        &(
            (data.len() +
                buffer_normals.len() +
                buffer_positions.len() +
                buffer_uvs.len() +
                buffer_indices.len() +
                16 + // Chunk headers
                12 // Top header
            ) as u32
        ).to_le_bytes()
    )?;

    file.write_all(&(data.len() as u32).to_le_bytes())?;
    file.write_all("JSON".as_bytes())?;
    file.write(data.as_bytes())?;

    file.write_all(&((buffer_positions.len() + buffer_normals.len() + buffer_uvs.len() + buffer_indices.len()) as u32).to_le_bytes())?;
    file.write_all("BIN".as_bytes())?;
    file.write(&[0])?;
    file.write_all(buffer_normals.as_slice())?;
    file.write_all(buffer_positions.as_slice())?;
    file.write_all(buffer_uvs.as_slice())?;
    file.write_all(buffer_indices.as_slice())?;
    return result::Result::Ok(());
}
