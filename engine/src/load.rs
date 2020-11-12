use crate::traits::{Normals, Position, TextCoords};
use glish_rs::buffer::VboSettings;
use obj::Obj;
use glish_rs::texture::{Texture, PathCubeMaps};
use std::path::Path;

pub fn load_obj_with_textures<T>(obj_data: &Obj<T>) -> (Vec<f32>, [VboSettings; 3])
where
    T: Position + Normals + TextCoords,
{
    let mut vertices = Vec::new();
    for vertex in &obj_data.vertices {
        let tex_coords = vertex.get_tex_coords();
        let position = vertex.get_position();
        let normals = vertex.get_normals();
        vertices.push(position[0]);
        vertices.push(position[1]);
        vertices.push(position[2]);
        vertices.push(normals[0]);
        vertices.push(normals[1]);
        vertices.push(normals[2]);
        vertices.push(tex_coords[0]);
        vertices.push(tex_coords[1]);
        vertices.push(tex_coords[2]);
    }
    (
        vertices,
        [
            VboSettings {
                location: 0,
                stride: 9,
                offset: 0,
                size: 3,
            },
            VboSettings {
                location: 1,
                stride: 9,
                offset: 3,
                size: 3,
            },
            VboSettings {
                location: 2,
                stride: 9,
                offset: 6,
                size: 3,
            },
        ],
    )
}
pub fn load_obj_vertices<T: Position + Normals>(obj_data: &Obj<T>) -> (Vec<f32>, [VboSettings; 2]) {
    let mut vertices = Vec::new();
    for vertex in &obj_data.vertices {
        let position = vertex.get_position();
        let normals = vertex.get_normals();
        vertices.push(position[0]);
        vertices.push(position[1]);
        vertices.push(position[2]);
        vertices.push(normals[0]);
        vertices.push(normals[1]);
        vertices.push(normals[2]);
    }
    (
        vertices,
        [
            VboSettings {
                location: 0,
                stride: 6,
                offset: 0,
                size: 3,
            },
            VboSettings {
                location: 1,
                stride: 6,
                offset: 3,
                size: 3,
            },
        ],
    )
}
