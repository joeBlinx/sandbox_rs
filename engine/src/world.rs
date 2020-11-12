use crate::camera::Camera;
use nalgebra_glm::{make_vec3};
use std::collections::HashMap;
use glish_rs::program::Program;
use glish_rs::texture::{Texture, PathCubeMaps};
use crate::mesh::Mesh;
use std::path::Path;
use glish_rs::shader::Shader;
pub struct WorldManager {
    cam: Camera,
    pub(crate) programs: HashMap<String, Program>,
    pub(crate) textures: HashMap<String, Texture>,
    pub(crate) meshs: HashMap<String, Mesh>,
}


impl WorldManager {

    pub fn add_program_from_shaders(&mut self, name: &str, shaders: &[Shader])-> Result<(), String>{
        let program = Program::from_shaders(shaders)?;
        self.programs.insert(name.to_owned(), program);
        Ok(())
    }

    pub fn add_textures(&mut self, name: &str, texture_path: &Path){
        match Texture::texture_2d_from_file(texture_path) {
            Ok(texture) => {
                self.textures.insert(String::from(name), texture);
            }
            Err(err) => eprintln!("{}", err),
        }
    }
    pub fn add_cube_map(&mut self, name: &str, folder_path: &Path){
        let path_3d_textures = PathCubeMaps {
            x: folder_path.join("right.jpg"),
            x_neg: folder_path.join("left.jpg"),
            y: folder_path.join("top.jpg"),
            y_neg: folder_path.join("bottom.jpg"),
            z: folder_path.join("front.jpg"),
            z_neg: folder_path.join("back.jpg"),
        };
        match Texture::texture_3d_from_files(path_3d_textures){
            Ok(texture) => {
                self.textures.insert(String::from(name), texture);
            }
            Err(err) => eprintln!("{}", err),
        }
    }

    pub fn add_mesh(&mut self, name: &str, mesh: Mesh){
        self.meshs.insert(name.to_owned(), mesh);
    }
}
impl Default for WorldManager {
    fn default() -> Self{
        WorldManager {
            cam: Camera::new(make_vec3(&[0.7, 1., 10.]), make_vec3(&[0., 0., 0.])),
            programs: HashMap::new(),
            textures: HashMap::new(),
            meshs: HashMap::new(),
        }
    }
}