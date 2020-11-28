use crate::mesh::Mesh;
use glish_rs::program::Program;
use glish_rs::shader::Shader;
use glish_rs::texture::{PathCubeMaps, Texture};
use std::collections::HashMap;
use std::path::Path;
use crate::resources::sprite_sheet::SpriteSheet;
use crate::reader_json::sprite_sheet::read_sprite_sheet;

pub struct RenderInfo {
    pub(crate) programs: HashMap<String, Program>,
    pub(crate) textures: HashMap<String, Texture>,
    pub(crate) meshs: HashMap<String, Mesh>,
    pub(crate) sprite_sheets: HashMap<String, SpriteSheet>,
}

impl RenderInfo {
    pub fn add_program_from_shaders(&mut self, name: &str, shaders: &[Shader]) {
        match Program::from_shaders(shaders) {
            Ok(program) => {
                self.programs.insert(name.to_owned(), program);
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        };
    }

    pub fn add_textures(&mut self, name: &str, texture_path: &Path) {
        match Texture::texture_2d_from_file(texture_path) {
            Ok(texture) => {
                self.textures.insert(String::from(name), texture);
            }
            Err(err) => eprintln!("{}", err),
        }
    }
    pub fn add_cube_map(&mut self, name: &str, folder_path: &Path) {
        let path_3d_textures = PathCubeMaps {
            x: folder_path.join("right.jpg"),
            x_neg: folder_path.join("left.jpg"),
            y: folder_path.join("top.jpg"),
            y_neg: folder_path.join("bottom.jpg"),
            z: folder_path.join("front.jpg"),
            z_neg: folder_path.join("back.jpg"),
        };
        match Texture::texture_3d_from_files(path_3d_textures) {
            Ok(texture) => {
                self.textures.insert(String::from(name), texture);
            }
            Err(err) => eprintln!("{}", err),
        }
    }

    pub fn add_mesh(&mut self, name: &str, mesh: Mesh) {
        self.meshs.insert(name.to_owned(), mesh);
    }

    pub fn add_sprite_sheet(&mut self, name:&str, path: &Path){
        match read_sprite_sheet(&path)
        {
            Ok(sprite_sheet) => {
                self.sprite_sheets.insert(String::from(name), sprite_sheet);
            },
            Err(err) => {
                eprintln!("{:#?}", err);
            },
        };
    }
}
impl Default for RenderInfo {
    fn default() -> Self {
        RenderInfo {
            programs: HashMap::new(),
            textures: HashMap::new(),
            meshs: HashMap::new(),
            sprite_sheets: HashMap::new(),
        }
    }
}
