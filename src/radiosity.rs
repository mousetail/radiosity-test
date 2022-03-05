use image::{Rgba, RgbaImage};
use crate::vector::{AXISES, Vec2, Vec3};

#[derive(Copy, Clone)]
pub struct Face {
    pub corners: [Vec3; 4],
    pub(crate) texture_position: [Vec2; 4],
    pub normal: Vec3,
    pub brightness: f32,
    pub(crate) id: u32,
}

impl Face {
    fn subdivide(&self) {}

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

pub fn simulate_radiosity(faces: &mut Vec<Face>, iterations: u8) -> RgbaImage {
    let occluder_faces = faces.clone();
    let mut texture: RgbaImage = RgbaImage::new(64, 64);
    let size = faces.len();
    //let mut faces1 = faces.clone();
    for i in 0..iterations {
        println!("Radiosity iteration {}, Faces: {}", i, size);
        let faces2 = faces.clone();
        for (face_index, face) in faces.iter_mut().enumerate() {
            if face_index % 20 == 0 {
                println!("| {}", faces2[0].brightness);
            }

            for face2 in &faces2 {
                if face.id == face2.id {
                    continue;
                }
                let position1 = face.center();
                let position2 = face2.center();

                let mut intersects = false;
                for face3 in &occluder_faces {
                    if face.id == face3.id || face2.id == face3.id {
                        continue;
                    }
                    if test_intersection(position1, position2, face3) {
                        intersects = true;
                        break;
                    }
                }

                if !intersects {
                    face.brightness += face2.brightness * (1. / face.distance_squared(&face2)) / 64. / 64.;
                }
            }
        }
    }

    for (index, face) in faces.iter().enumerate() {
        let brightness = face.brightness;
        println!("setting pixel {} {} to value {}", index as u32 / 64,
                 index as u32 % 64,
                 brightness);
        texture.put_pixel(
            index as u32 / 64,
            index as u32 % 64,
            [brightness as u8, (brightness * 2.) as u8, (brightness * 4.) as u8, 255].into(),
        )
    }

    return texture;
}
