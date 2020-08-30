use crate::traits::Draw;
use crate::{camera::Camera, load};
use glish_rs::buffer::Vao;
use glish_rs::program::Program;
use glish_rs::shader::Shader;
use glish_rs::texture::{PathCubeMaps, Texture};
use glish_rs::{buffer, utils};
use obj::{load_obj, Obj};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct Skybox {
    vao: Vao,
    opengl_prog: Program,
    texture: Texture,
    number_indices: usize,
}

fn create_skybox_textures(skybox_texture_folder: &Path) -> Result<Texture, String> {
    let path_3d_textures = PathCubeMaps {
        x: skybox_texture_folder.join("right.jpg"),
        x_neg: skybox_texture_folder.join("left.jpg"),
        y: skybox_texture_folder.join("top.jpg"),
        y_neg: skybox_texture_folder.join("bottom.jpg"),
        z: skybox_texture_folder.join("front.jpg"),
        z_neg: skybox_texture_folder.join("back.jpg"),
    };
    Texture::texture_3d_from_files(path_3d_textures)
}
impl Skybox {
    pub fn new(skybox_texture_folder: &Path) -> Result<Self, String> {
        let input = BufReader::new(File::open("assets/obj/cube.obj").unwrap());
        let obj_data: Obj = load_obj(input).unwrap();
        let (vertices, vbo_settings) = load::load_obj_vertices(&obj_data);
        let number_indices_cube = obj_data.indices.len();
        let vbo_indices_cube = buffer::Vbo::create_elements(obj_data.indices);
        let vbo_cube = buffer::Vbo::create(vertices);
        let vao = buffer::Vao::create(vbo_indices_cube, vbo_cube, &vbo_settings);

        let frag_skybox = Shader::from_frag_file(Path::new("assets/shader/fragment/skybox.frag")).unwrap();
        let vert_skybox = Shader::from_vert_file(Path::new("assets/shader/vertex/skybox.vert")).unwrap();
        let skybox_prog = Program::from_shaders(&[vert_skybox, frag_skybox]).unwrap();
        let texture = create_skybox_textures(skybox_texture_folder)?;

        Ok(Skybox {
            vao,
            opengl_prog: skybox_prog,
            texture,
            number_indices: number_indices_cube,
        })
    }
}
impl Draw for Skybox {
    fn draw(&mut self, cam: &Camera) {
        unsafe {
            gl::DepthFunc(gl::LEQUAL);
        }
        self.vao.bind();
        self.opengl_prog.set_uni("proj", cam.get_proj());
        self.opengl_prog.set_uni("view", cam.get_view());
        self.opengl_prog.set_uni("cubemap", 1);
        self.texture.active(1);
        utils::draw_elements(self.number_indices);
        unsafe {
            gl::DepthFunc(gl::LESS);
        }
    }
}
