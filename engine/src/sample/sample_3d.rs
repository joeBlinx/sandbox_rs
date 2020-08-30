use crate::camera::Camera;
use crate::load;
use crate::traits::Draw;
use glish_rs::buffer::Vao;
use glish_rs::program::Program;
use glish_rs::shader::Shader;
use glish_rs::texture::Texture;
use glish_rs::{buffer, utils};
use obj::{load_obj, Obj, TexturedVertex};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct Sample3d {
    vao: Vao,
    opengl_prog: Program,
    texture: Texture,
    number_indices: usize,
}

impl Sample3d {
    pub fn new(obj_path: &Path, texture_path: &Path) -> Result<Self, String> {
        let vert_shader = Shader::from_vert_file(&Path::new("assets/triangle.vert")).unwrap();
        let frag_shader = Shader::from_frag_file(&Path::new("assets/triangle.frag")).unwrap();
        let shader_program = Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

        let input = BufReader::new(File::open(obj_path).unwrap());
        let obj_data: Obj<TexturedVertex> = load_obj(input).unwrap();

        let (vertices, vbo_settings) = load::load_obj_with_textures(&obj_data);
        let number_indices = obj_data.indices.len();
        let vbo_indices = buffer::Vbo::create_elements(obj_data.indices);
        let vbo = buffer::Vbo::create(vertices);
        let vao = buffer::Vao::create(vbo_indices, vbo, &vbo_settings);

        let texture = Texture::texture_2d_from_file(texture_path)?;
        Ok(Sample3d {
            vao,
            opengl_prog: shader_program,
            texture,
            number_indices,
        })
    }
}

impl Draw for Sample3d {
    fn draw(&mut self, cam: &Camera) {
        self.vao.bind();
        self.opengl_prog.set_used();
        self.opengl_prog.set_uni("pos_cam", cam.get_position());
        self.opengl_prog
            .set_uni("cam", cam.get_proj() * cam.get_view());
        self.opengl_prog.set_uni("lava_texture", 1);
        self.texture.active(1);
        utils::draw_elements(self.number_indices);
    }
}
