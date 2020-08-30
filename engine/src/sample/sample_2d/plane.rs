use crate::traits::Draw;
use std::collections::HashMap;
use glish_rs::{
    texture::Texture,
    buffer::Vao,
    buffer::VboSettings,
    buffer,
    program::Program,
    shader::Shader,
    shader,
    utils
};
use gl::types::GLenum;
use crate::camera::Camera;
use std::path::Path;

pub struct Plane{
    vao: Vao,
    opengl_prog: Program,
    textures: HashMap<String, Texture>,
    shaders: HashMap<GLenum, Shader>,
    number_indices: usize,
}
impl Default for Plane{
    fn default() -> Self{
        let vertices = [
          -1., -1., 0., 1., //1    
          1., 1., 1., 0., //2
          -1., 1., 0., 0., //3
          1., -1., 1., 1. //4
        ];
        let indices = [
            0, 1, 2,
            0, 1, 3
        ];
        let vbo_indices = buffer::Vbo::create_elements(indices.to_vec());
        let vbo_settings = [
            VboSettings{
                location: 0,
                size: 2,
                stride: 4,
                offset: 0
            },
            VboSettings{
                location: 2,
                size: 2,
                stride: 4,
                offset: 2
            }
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
}
impl Plane{
    pub fn new_with_shaders(shader_settings: &[shader::ShaderSettings])-> Self{
        let mut plane = Plane::default();
        for  settings in shader_settings.iter(){
            plane.add_shader(settings.stage, settings.path);
        }
        plane
    }
    pub fn add_shader(&mut self, shader_type: GLenum, shader_path: &Path){
        let shader = Shader::from_file(shader_path, shader_type);
        match shader{
            Ok(shader) => {self.shaders.insert(shader_type, shader);},
            Err(err) => eprintln!("{}", err),
        };
        if self.shaders.contains_key(&gl::VERTEX_SHADER) && self.shaders.contains_key(&gl::FRAGMENT_SHADER){
            let mut shaders = Vec::new();
            for shader in self.shaders.values(){
                shaders.push(shader.clone());
            }
            match Program::from_shaders(&shaders[..]){
                Ok(prog) => self.opengl_prog = prog,
                Err(err) => eprintln!("{}", err),
            }
        }
    }
    pub fn add_textures(&mut self, texture_key: &str, texture_path: &Path){
       match Texture::texture_2d_from_file(texture_path){
           Ok(texture) => {
               self.textures.insert(String::from(texture_key), texture);
           },
           Err(err) => eprintln!("{}", err)
        }
    }
}
impl Draw for Plane{
    fn draw(&mut self, cam: &Camera){
        self.vao.bind();
        self.opengl_prog.set_used();
        self.opengl_prog.set_uni("pos_cam", cam.get_position());
        self.opengl_prog
            .set_uni("cam", cam.get_proj() * cam.get_view());
        {
            let mut i = 0 as u32;
            for (key, texture) in self.textures.iter(){
                self.opengl_prog.set_uni(&key[..], i as i32);
                texture.active(i);
                i += 1;
            }
        }
        utils::draw_elements(self.number_indices);
    }
}