use glish_rs::buffer::{Vao, VboSettings};
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use obj::{Obj, TexturedVertex, load_obj};
use crate::load;
use glish_rs::buffer;
pub struct SkyBox;
pub struct Mesh{
    pub (crate) vao: Vao,
    pub (crate) number_indices: usize
}
impl Mesh{
    pub fn from_obj_file(obj_path: &Path) -> Result<Mesh, std::io::Error> {
        let file = File::open(obj_path)?;
        let input = BufReader::new(file);
        let obj_data: Obj<TexturedVertex> = load_obj(input).unwrap();
        let (vertices, vbo_settings) = load::load_obj_with_textures(&obj_data);
        let number_indices = obj_data.indices.len();
        let vbo_indices = buffer::Vbo::create_elements(obj_data.indices);
        let vbo = buffer::Vbo::create(vertices);
        let vao = buffer::Vao::create(vbo_indices, vbo, &vbo_settings);
        let number_indices = number_indices;
        Ok(
            Mesh{
                vao,
                number_indices
            }
        )
    }

    pub fn create_plane() -> Self{
        let vertices = [
            -1., -1., 0., 1., 0., 0., 1., //1
            1., 1., 1., 0., 0., 0., 1., //2
            -1., 1., 0., 0., 0., 0., 1., //3
            1., -1., 1., 1., 0., 0., 1., //4
        ];
        let indices = [0, 1, 2, 0, 1, 3];
        let vbo_indices = buffer::Vbo::create_elements(indices.to_vec());
        let vbo_settings = [
            VboSettings {
                location: 0, // position
                size: 2,
                stride: 7,
                offset: 0,
            },
            VboSettings {// uv
                location: 1,
                size: 2,
                stride: 7,
                offset: 2,
            },
            VboSettings { // normal
                location: 2,
                size: 3,
                stride: 7,
                offset: 4,
            },
        ];
        let vbo = buffer::Vbo::create(vertices.to_vec());
        let vao = buffer::Vao::create(vbo_indices, vbo, &vbo_settings);

        Self {
            vao,
            number_indices: indices.len(),
        }
    }
}