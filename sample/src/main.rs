mod camera;
mod handle_event;
mod load;
mod traits;
mod window;
use glish_rs;
use crate::handle_event::HandleEvent;
use nalgebra_glm::make_vec3;
use obj::{load_obj, Obj, TexturedVertex};
use std::ffi::CString;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use glish_rs::{shader, program, buffer, utils};
use glish_rs::shader::Shader;
use glish_rs::program::Program;
use glish_rs::texture::{Texture, PathCubeMaps};

extern crate gl;
extern crate rand;
extern crate sdl2;

fn main() {
    let window = window::Window::new();
    let sdl = window.sdl();
    let mut event = sdl.event_pump().unwrap();

    let vert_shader = shader::Shader::from_vert_file(&Path::new("assets/triangle.vert")).unwrap();

    let frag_shader = shader::Shader::from_frag_file(&Path::new("assets/triangle.frag")).unwrap();
    let mut shader_program = program::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    let input = BufReader::new(File::open("assets/sphere.obj").unwrap());
    let obj_data: Obj<TexturedVertex> = load_obj(input).unwrap();

    shader_program.set_used();
    let mut cam = camera::Camera::new(make_vec3(&[0.7, 1., -10.]), make_vec3(&[0., 0., 0.]));
    shader_program.set_uni("cam", cam.get_proj() * cam.get_view());
    let lava_texture =
       Texture::texture_2d_from_file(Path::new("assets/cube_texture.png"));
    let (vertices, vbo_settings) = load::load_obj_with_textures(&obj_data);
    let number_indices = obj_data.indices.len();
    let vbo_indices = buffer::Vbo::create_elements(obj_data.indices);
    let vbo = buffer::Vbo::create(vertices);
    let vao = buffer::Vao::create(vbo_indices, vbo, &vbo_settings);

    let path_3d_textures = PathCubeMaps {
        x: PathBuf::from("assets/skybox/right.jpg"),
        x_neg: PathBuf::from("assets/skybox/left.jpg"),
        y: PathBuf::from("assets/skybox/top.jpg"),
        y_neg: PathBuf::from("assets/skybox/bottom.jpg"),
        z: PathBuf::from("assets/skybox/front.jpg"),
        z_neg: PathBuf::from("assets/skybox/back.jpg"),
    };

    let frag_skybox = Shader::from_frag_file(Path::new("assets/skybox.frag")).unwrap();
    let vert_skybox = Shader::from_vert_file(Path::new("assets/skybox.vert")).unwrap();
    let mut skybox_prog = Program::from_shaders(&[vert_skybox, frag_skybox]).unwrap();
    let skybox_texture = Texture::texture_3d_from_files(path_3d_textures);

    let input = BufReader::new(File::open("assets/cube.obj").unwrap());
    let obj_data: Obj = load_obj(input).unwrap();
    let (vertices, vbo_settings) = load::load_obj_vertices(&obj_data);
    let number_indices_cube = obj_data.indices.len();
    let vbo_indices_cube = buffer::Vbo::create_elements(obj_data.indices);
    let vbo_cube = buffer::Vbo::create(vertices);
    let vao_cube = buffer::Vao::create(vbo_indices_cube, vbo_cube, &vbo_settings);
    vao.bind();
    'main: loop {
        window.clear();
        for event in event.poll_iter() {
            cam.handle_event(&event);
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::KeyDown { keycode, .. } => {
                    let keycode = keycode.unwrap();
                    match keycode {
                        sdl2::keyboard::Keycode::Escape => break 'main,
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        vao.bind();
        shader_program.set_uni("pos_cam", cam.get_position());
        shader_program.set_uni("cam", cam.get_proj() * cam.get_view());
        shader_program.set_uni("lava_texture", 1);
        lava_texture.active(1);
        utils::draw_elements(number_indices);

        unsafe {
            gl::DepthFunc(gl::LEQUAL);
        }
        vao_cube.bind();
        skybox_prog.set_uni("proj", cam.get_proj());
        skybox_prog.set_uni("view", cam.get_view());
        skybox_prog.set_uni("cubemap", 1);
        skybox_texture.active(1);
        utils::draw_elements(number_indices_cube);
        unsafe {
            gl::DepthFunc(gl::LESS);
        }

        let ten_millis = std::time::Duration::from_millis(17);
        std::thread::sleep(ten_millis);
        window.refresh();
    }
}
