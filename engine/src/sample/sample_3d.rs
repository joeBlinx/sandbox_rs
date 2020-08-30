use crate::camera::Camera;
use crate::load;
use crate::traits::Draw;
use gl::types::*;
use glish_rs::buffer::Vao;
use glish_rs::buffer::VboSettings;
use glish_rs::program::Program;
use glish_rs::shader::Shader;
use glish_rs::texture::Texture;
use glish_rs::{buffer, utils};
use obj::{load_obj, Obj, TexturedVertex};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct Sample3d {
    vao: Vao,
    opengl_prog: Program,
    textures: HashMap<String, Texture>,
    shaders: HashMap<GLenum, Shader>,
    number_indices: usize,
}

impl Sample3d {
    pub fn from_obj_file(obj_path: &Path) -> Result<Self, String> {
        let mut sample = Sample3d::default();
        match sample.add_obj_file(obj_path) {
            Err(err) => Err(err.to_string()),
            _ => Ok(sample),
        }
    }
    pub fn add_obj_file(&mut self, obj_path: &Path) -> Result<bool, std::io::Error> {
        let file = File::open(obj_path)?;
        let input = BufReader::new(file);
        let obj_data: Obj<TexturedVertex> = load_obj(input).unwrap();
        let (vertices, vbo_settings) = load::load_obj_with_textures(&obj_data);
        let number_indices = obj_data.indices.len();
        let vbo_indices = buffer::Vbo::create_elements(obj_data.indices);
        let vbo = buffer::Vbo::create(vertices);
        let vao = buffer::Vao::create(vbo_indices, vbo, &vbo_settings);
        self.vao = vao;
        self.number_indices = number_indices;

        Ok(true)
    }
    pub fn create_plane() -> Self {
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
                location: 0,
                size: 2,
                stride: 7,
                offset: 0,
            },
            VboSettings {
                location: 2,
                size: 2,
                stride: 7,
                offset: 2,
            },
            VboSettings {
                location: 1,
                size: 3,
                stride: 7,
                offset: 4,
            },
        ];
        let vbo = buffer::Vbo::create(vertices.to_vec());
        let vao = buffer::Vao::create(vbo_indices, vbo, &vbo_settings);

        Self {
            vao,
            opengl_prog: Program::default(),
            textures: HashMap::new(),
            shaders: HashMap::new(),
            number_indices: indices.len(),
        }
    }
    pub fn add_shader(&mut self, shader_type: GLenum, shader_path: &Path) {
        let shader = Shader::from_file(shader_path, shader_type);
        match shader {
            Ok(shader) => {
                self.shaders.insert(shader_type, shader);
            }
            Err(err) => eprintln!("{}", err),
        };
        if self.shaders.contains_key(&gl::VERTEX_SHADER)
            && self.shaders.contains_key(&gl::FRAGMENT_SHADER)
        {
            let mut shaders = Vec::new();
            for shader in self.shaders.values() {
                shaders.push(shader.clone());
            }
            match Program::from_shaders(&shaders[..]) {
                Ok(prog) => self.opengl_prog = prog,
                Err(err) => eprintln!("{}", err),
            }
        }
    }
    pub fn add_texture(&mut self, texture_key: &str, texture_path: &Path) {
        match Texture::texture_2d_from_file(texture_path) {
            Ok(texture) => {
                self.textures.insert(String::from(texture_key), texture);
            }
            Err(err) => eprintln!("{}", err),
        }
    }
}

impl Draw for Sample3d {
    fn draw(&mut self, cam: &Camera) {
        self.vao.bind();
        self.opengl_prog.set_used();
        self.opengl_prog.set_uni("pos_cam", cam.get_position());
        self.opengl_prog
            .set_uni("cam", cam.get_proj() * cam.get_view());
        {
            let mut i = 0 as u32;
            for (key, texture) in self.textures.iter() {
                self.opengl_prog.set_uni(&key[..], i as i32);
                texture.active(i);
                i += 1;
            }
        }
        utils::draw_elements(self.number_indices);
    }
}

impl Default for Sample3d {
    fn default() -> Self {
        Sample3d {
            vao: Vao::default(),
            opengl_prog: Program::default(),
            textures: HashMap::new(),
            shaders: HashMap::new(),
            number_indices: 0,
        }
    }
}
