use image::{Pixel, Rgb, Rgba, RgbaImage};
use crate::radiosity_color::RadiosityColor;
use crate::vector::{Axis, AXISES, Vec2, Vec3};

#[derive(Copy, Clone, Debug)]
pub struct Face {
    pub corners: [Vec3; 4],
    pub(crate) texture_position: [Vec2; 4],
    pub normal: Vec3,
    pub brightness: [f32; 3],
    pub last_iteration_brightness: [f32; 3],
    pub(crate) id: u32,
    pub(crate) color: Rgba<u8>,
}

fn get_subdivisions() -> [[(usize, usize); 4]; 4] {
    return [
        [
            (0, 0),
            (0, 1),
            (0, 2),
            (0, 3)
        ],
        [
            (0, 1),
            (1, 1),
            (0, 3),
            (1, 3)
        ],
        [
            (0, 2),
            (0, 3),
            (2, 2),
            (2, 3)
        ],
        [
            (0, 3),
            (1, 3),
            (2, 3),
            (3, 3)
        ]
    ];
}

struct FaceRefections {
    iteration: u8,
    face: Face
}

impl FaceRefections {
    fn new(face: Face) -> FaceRefections {
        FaceRefections {
            face,
            iteration: 0
        }
    }
}

impl Iterator for FaceRefections {
    type Item = Face;

    fn next(&mut self) -> Option<Self::Item> {

        let mut face2 = self.face;
        if (self.iteration & 4) != 0 {
            return Option::None
        }

        // if (self.iteration & 4) != 0 {
        //     face2 = face2.reflect(Axis::Z);
        // }
        if (self.iteration & 2) != 0 {
            face2 = face2.reflect(Axis::X);
        }
        if (self.iteration & 1) != 0 {
            face2 = face2.reflect(Axis::Z);
        }
        self.iteration += 1;

        return Some(face2)
    }
}

impl Face {
    fn subdivide(&self) -> [Self; 4] {
        get_subdivisions().map(
            |subdiv| Face {
                corners: subdiv.map(|indexes| (self.corners[indexes.0] + self.corners[indexes.1]) * 0.5),
                texture_position: subdiv.map(|indexes| (self.texture_position[indexes.0] + self.texture_position[indexes.1]) * 0.5),
                normal: self.normal,
                brightness: self.brightness,
                last_iteration_brightness: self.last_iteration_brightness,
                id: self.id,
                color: self.color,
            }
        )
    }

    fn center(&self) -> Vec3 {
        return Vec3 {
            x: (self.corners[0].x + self.corners[1].x + self.corners[2].x + self.corners[3].x) / 4.,
            y: (self.corners[0].y + self.corners[1].y + self.corners[2].y + self.corners[3].y) / 4.,
            z: (self.corners[0].z + self.corners[1].z + self.corners[2].z + self.corners[3].z) / 4.,
        };
    }

    fn size(&self) -> Vec3 {
        return Vec3 {
            x: self.corners[0].x.max(self.corners[1].x).max(self.corners[2].x).max(self.corners[3].x),
            y: self.corners[0].y.max(self.corners[1].y).max(self.corners[2].y).max(self.corners[3].y),
            z: self.corners[0].z.max(self.corners[1].z).max(self.corners[2].z).max(self.corners[3].z),
        } - Vec3 {
            x: self.corners[0].x.min(self.corners[1].x).min(self.corners[2].x).min(self.corners[3].x),
            y: self.corners[0].y.min(self.corners[1].y).min(self.corners[2].y).min(self.corners[3].y),
            z: self.corners[0].z.min(self.corners[1].z).min(self.corners[2].z).min(self.corners[3].z),
        };
    }

    fn distance_squared(&self, other: &Self) -> f32 {
        let c1 = self.center();
        let c2 = other.center();
        return c1.distance_squared(&c2);
    }

    fn reflect(&self, axis: Axis) -> Face {
        Face {
            corners: self.corners.map(|x| x.reflect(axis)),
            texture_position: self.texture_position,
            normal: self.normal.reflect(axis),
            brightness: self.brightness,
            last_iteration_brightness: self.last_iteration_brightness,
            id: self.id,
            color: self.color
        }
    }
}

fn test_intersection(p1: Vec3, p2: Vec3, face: &Face) -> bool {
    let box_center = face.center();
    let box_size = face.size();

    let line_length = p2 - p2;
    let min = box_center - box_size * 0.5;
    let max = box_center + box_size * 0.5;

    let begin_to_min = min - p1;
    let begin_to_max = max - p1;

    let mut near = -1.0;
    let mut far = 3.0;

    for axis in AXISES {
        if line_length.get_axis(axis) == 0. {
            if begin_to_min.get_axis(axis) > 0. || begin_to_max.get_axis(axis) < 0. {
                return false;
            }
        } else {
            let t1 = begin_to_min.get_axis(axis) / line_length.get_axis(axis);
            let t2 = begin_to_max.get_axis(axis) / line_length.get_axis(axis);

            let t_min = t1.min(t2);
            let t_max = t1.max(t2);

            if t_min > near {
                near = t_min
            }
            if t_max > far {
                far = t_max
            }
            if near > far || far < 0. {
                return false;
            }
        }
    }
    return (near > 0. && near < 1.) || (far > 0. && far < 1.);
}

pub fn radiosity_subdivide(faces: &mut Vec<Face>, iterations: u8, subdivisions: u8) -> RgbaImage {
    let mut faces2 = faces.iter().map(|x| x.subdivide()).flatten().collect();
    simulate_radiosity(&mut faces2, iterations)
}

pub fn simulate_radiosity(faces: &mut Vec<Face>, iterations: u8) -> RgbaImage {
    println!("Face original: {:?}", faces[0]);
    println!("Face: {:?}", faces[0].subdivide());
    //let occluder_faces = faces.clone();
    let mut texture: RgbaImage = RgbaImage::new(128, 128);
    let size = faces.len();

    for face in faces.iter_mut() {
        face.last_iteration_brightness = face.brightness;
    }
    //let mut faces1 = faces.clone();
    for i in 0..iterations {
        println!("Radiosity iteration {}, Faces: {}", i, size);
        let faces2 = faces.clone();
        for face in faces.iter_mut() {
            for i in 0..3 {
                face.brightness[i] += face.last_iteration_brightness[i]
            }
            face.last_iteration_brightness = [0.; 3];
        }

        for (face_index, face) in faces.iter_mut().enumerate() {
            for face2_base in &faces2 {
                if face.id == face2_base.id {
                    continue;
                }
                if face2_base.last_iteration_brightness == [0., 0., 0.] {
                    continue;
                }
                for face2 in FaceRefections::new(*face2_base) {
                    let position1 = face.center();
                    let position2 = face2.center();
                    let difference = (position1 - position2).normalize();

                    // let mut intersects = false;
                    // for face3 in &occluder_faces {
                    //     if face.id == face3.id || face2.id == face3.id {
                    //         continue;
                    //     }
                    //     if test_intersection(position1, position2, face3) {
                    //         intersects = true;
                    //         break;
                    //     }
                    // }
                    assert!(face.distance_squared(&face2) >= 1. / 4096., "distance equals: {}, face 1 ID: {} {:?}, face 2 ID: {} {:?}", face.distance_squared(&face2), face.id, face.center(), face2.id, face2.center());

                    let factor = (difference.dot(&face.normal)).max(0.) * (-difference.dot(&face2.normal)).max(0.);
                    //if !intersects {
                    for i in 0..3 {
                        face.last_iteration_brightness[i] += (face.color[i] as f32 / 256.)
                            * face2.last_iteration_brightness[i]
                            * (1. / face.distance_squared(&face2)) / 32. / 32.
                            * factor;
                    }
                }
                //}
            }
        }
    }

    for face in faces.iter_mut() {
        for i in 0..3 {
            face.brightness[i] += face.last_iteration_brightness[i]
        }
    }

    for (index, face) in faces.iter().enumerate() {
        let brightness = face.brightness;
        // println!("setting pixel {} {} to value {}", index as u32 / 64,
        //          index as u32 % 64,
        //          brightness);
        let mut color = Rgb::to_rgba(&brightness.map(|x| (x * 256.) as u8).into());
        color[3] = 255;
        texture.put_pixel(
            ((face.texture_position[0].x + face.texture_position[3].x) * 0.5 * texture.width() as f32) as u32,
            ((face.texture_position[0].y + face.texture_position[3].y) * 0.5 * texture.height() as f32) as u32,
            color,
        )
    }

    return texture;
}
